# Expression interpreter

This example builds a modular interpreter for a small arithmetic language, where each operator is its own type and each operation over the language — evaluation, conversion to Lisp — is a separate provider, so variants and operations can both be added without editing existing code. It progresses from the closed enum-and-`match` form, through per-variant evaluation providers wired by input dispatch, to a second operation, a generalized operator provider, code-based dispatch between operations, and finally an extended language with new variants. It is a template for any recursive data type — expression trees, JSON values, syntax trees — that must stay open to new cases and new traversals at once, the classic [expression problem](https://en.wikipedia.org/wiki/Expression_problem).

The concepts each step demonstrates are documented in full in the reference; this example only notes which one is in play and links to it:

- handling each variant of an enum independently — [extensible variants](../concepts/extensible-variants.md) and the [extensible visitor pattern](../concepts/dispatching.md)
- exposing an enum as a sum of named variants — [`#[derive(HasFields)]`](../reference/derives/derive_has_fields.md), [`#[derive(FromVariant)]`](../reference/derives/derive_from_variant.md), [`#[derive(ExtractField)]`](../reference/derives/derive_extract_field.md)
- the computation components — [`Computer` / `CanCompute`](../reference/components/computer.md) and its by-reference variant `ComputerRef`
- writing a per-variant provider — [`#[cgp_impl]`](../reference/macros/cgp_impl.md)
- routing on the input variant and on the operation — [`UseDelegate`](../reference/providers/use_delegate.md) and [dispatching](../concepts/dispatching.md)
- the variant dispatcher — [`MatchWithValueHandlers`](../reference/providers/dispatch_combinators.md)
- constructing part of a target enum — [`CanUpcast`](../reference/traits/cast.md)
- abstract output types per context — [`#[cgp_type]`](../reference/macros/cgp_type.md) and [`UseType`](../reference/providers/use_type.md)

All snippets assume `use cgp::prelude::*;`, with the computation items from `cgp::extra::handler`, the dispatchers from `cgp::extra::dispatch`, and `CanUpcast` from `cgp::core::field::impls`.

## The closed interpreter

The conventional way to model the language is one enum with a `match`-based function per operation:

```rust
pub enum Expr {
    Plus(Box<Expr>, Box<Expr>),
    Times(Box<Expr>, Box<Expr>),
    Literal(u64),
}

pub fn eval(expr: Expr) -> u64 {
    match expr {
        Expr::Plus(a, b) => eval(*a) + eval(*b),
        Expr::Times(a, b) => eval(*a) * eval(*b),
        Expr::Literal(value) => value,
    }
}
```

This is concise but closed in two directions. Adding a variant forces every function that matches on `Expr` to change, and the recursive structure means even a helper like `eval_plus` must still mention `Expr`. A real expression type such as `syn::Expr` has dozens of variants and many operations, so this coupling becomes the central obstacle the rest of the example removes.

## Variants as standalone types

The first move is to give each operator its own type, generic over the expression it nests, rather than burying its shape in the enum:

```rust
#[derive(Debug, Eq, PartialEq, HasField)]
pub struct Plus<Expr> {
    pub left: Box<Expr>,
    pub right: Box<Expr>,
}

#[derive(Debug, Eq, PartialEq, HasField)]
pub struct Times<Expr> {
    pub left: Box<Expr>,
    pub right: Box<Expr>,
}

#[derive(Debug, Eq, PartialEq)]
pub struct Literal<T>(pub T);
```

Each operator is now a reusable building block parameterized by the broader expression type, which is what lets the same `Plus` appear in several languages. `Plus` and `Times` derive [`#[derive(HasField)]`](../reference/derives/derive_has_field.md) so their `left` and `right` fields can be read generically later.

## Evaluating one variant

Evaluation is a [`Computer`](../reference/components/computer.md) — CGP's component for a synchronous, pure computation whose consumer trait is `CanCompute`. One provider handles addition, recursing into the operands through the context's own evaluation:

```rust
#[cgp_impl(new EvalAdd)]
impl<Code, MathExpr, Output> Computer<Code, Plus<MathExpr>>
where
    Self: CanCompute<Code, MathExpr, Output = Output>,
    Output: Add<Output = Output>,
{
    type Output = Output;

    fn compute(&self, code: PhantomData<Code>, Plus { left, right }: Plus<MathExpr>) -> Self::Output {
        let output_a = self.compute(code, *left);
        let output_b = self.compute(code, *right);
        output_a + output_b
    }
}
```

`EvalAdd` is written with [`#[cgp_impl]`](../reference/macros/cgp_impl.md) and is completely decoupled: it knows nothing of the concrete expression enum, only that the context can evaluate a nested `MathExpr` to some `Output` that supports `Add`. Multiplication is identical with `Mul`, and the literal case is the base of the recursion — it just returns its inner value, needing nothing from the context:

```rust
#[cgp_impl(new EvalLiteral)]
impl<Code, T> Computer<Code, Literal<T>> {
    type Output = T;

    fn compute(&self, _code: PhantomData<Code>, Literal(value): Literal<T>) -> T {
        value
    }
}
```

Each provider lives on its own and could be defined in a separate crate; nothing ties `EvalAdd`, `EvalMultiply`, and `EvalLiteral` together until a context composes them.

## Assembling the evaluator

The concrete enum wraps the standalone operator types and derives the [extensible-variant](../concepts/extensible-variants.md) machinery so it can be taken apart generically:

```rust
pub type Value = u64;

#[derive(Debug, HasFields, FromVariant, ExtractField)]
pub enum MathExpr {
    Plus(Plus<MathExpr>),
    Times(Times<MathExpr>),
    Literal(Literal<Value>),
}
```

The context is an empty struct whose only job is to wire each input type to its provider. [`UseInputDelegate`](../reference/providers/use_delegate.md) keys the `Computer` lookup on the *input* type, so `Plus<MathExpr>` routes to `EvalAdd`, `Literal<Value>` to `EvalLiteral`, and the whole enum to a dispatcher:

```rust
pub struct Interpreter;

delegate_components! {
    Interpreter {
        ComputerComponent:
            UseInputDelegate<new EvalComponents {
                MathExpr: DispatchEval,
                Plus<MathExpr>: EvalAdd,
                Times<MathExpr>: EvalMultiply,
                Literal<Value>: EvalLiteral,
            }>,
    }
}

#[cgp_impl(new DispatchEval)]
impl<Code> Computer<Code, MathExpr> for Interpreter {
    type Output = Value;

    fn compute(context: &Interpreter, code: PhantomData<Code>, expr: MathExpr) -> Self::Output {
        <MatchWithValueHandlers>::compute(context, code, expr)
    }
}
```

The whole `MathExpr` enum is handled by `DispatchEval`, a context-specific provider that defers to [`MatchWithValueHandlers`](../reference/providers/dispatch_combinators.md) — the variant dispatcher that derives one handler per variant from the enum's own variant list and runs them as a match, described in [dispatching](../concepts/dispatching.md). The thin `DispatchEval` wrapper is needed to break a trait-resolution cycle: wiring `MatchWithValueHandlers` directly for `MathExpr` would require the compiler to resolve the per-variant providers, which themselves route back through the dispatcher. Marking the trait implemented in the wrapper's body breaks the cycle.

## A second operation over the same language

Converting an expression to a Lisp [S-expression](https://en.wikipedia.org/wiki/S-expression) is a second operation, and adding it must not touch the evaluator. It uses `ComputerRef`, the by-reference variant of `Computer`, so the expression can still be used afterward, and it targets a separate `LispExpr` enum — making this a "double" expression problem, decoupled from both the source and the target type. The target type stays abstract through a [`#[cgp_type]`](../reference/macros/cgp_type.md) component:

```rust
#[cgp_type]
pub trait HasLispExprType {
    type LispExpr;
}

#[cgp_impl(new PlusToLisp)]
impl<Code, MathExpr, LispExpr> ComputerRef<Code, Plus<MathExpr>>
where
    Self: HasLispExprType<LispExpr = LispExpr> + CanComputeRef<Code, MathExpr, Output = LispExpr>,
    LispSubExpr<LispExpr>: CanUpcast<LispExpr>,
{
    type Output = LispExpr;

    fn compute_ref(&self, code: PhantomData<Code>, Plus { left, right }: &Plus<MathExpr>) -> Self::Output {
        let expr_a = self.compute_ref(code, left);
        let expr_b = self.compute_ref(code, right);
        let ident = LispSubExpr::Ident(Ident("+".to_owned())).upcast(PhantomData);

        LispSubExpr::List(List(vec![ident.into(), expr_a.into(), expr_b.into()])).upcast(PhantomData)
    }
}
```

`PlusToLisp` only needs to build two kinds of `LispExpr` — a list and an identifier — so rather than depend on the full target enum it defines a small local enum with just those variants and [upcasts](../reference/traits/cast.md) into the full `LispExpr`:

```rust
#[derive(HasFields, ExtractField, FromVariant)]
enum LispSubExpr<Expr> {
    List(List<Expr>),
    Ident(Ident),
}
```

This is the variant-side analog of reading only the fields you need from a struct: `CanUpcast` constructs the parts of an enum a provider cares about without binding it to the entire definition. Wiring the new operation adds a `ComputerRefComponent` table and a `DispatchToLisp` wrapper alongside the existing evaluator, and binds the abstract `LispExpr` type to the concrete enum with [`UseType`](../reference/providers/use_type.md):

```rust
delegate_components! {
    Interpreter {
        MathExprTypeProviderComponent: UseType<MathExpr>,
        LispExprTypeProviderComponent: UseType<LispExpr>,
        ComputerComponent:
            UseInputDelegate<new EvalComponents {
                MathExpr: DispatchEval,
                Plus<MathExpr>: EvalAdd,
                Times<MathExpr>: EvalMultiply,
                Literal<Value>: EvalLiteral,
            }>,
        ComputerRefComponent:
            UseInputDelegate<new ToLispComponents {
                MathExpr: DispatchToLisp,
                Literal<Value>: LiteralToLisp,
                Plus<MathExpr>: PlusToLisp,
                Times<MathExpr>: TimesToLisp,
            }>,
    }
}
```

The evaluator wiring is untouched; the conversion is added purely by extension.

## One provider for every binary operator

`PlusToLisp` and `TimesToLisp` differ only in the operator symbol, so they collapse into a single provider parameterized by the operator. The operator is a type-level string, and the operand fields are read through a getter that any binary struct satisfies:

```rust
#[cgp_auto_getter]
pub trait BinarySubExpression<Expr> {
    fn left(&self) -> &Box<Expr>;
    fn right(&self) -> &Box<Expr>;
}

#[cgp_impl(new BinaryOpToLisp<Operator>)]
impl<Code, MathExpr, MathSubExpr, LispExpr, Operator> ComputerRef<Code, MathSubExpr>
where
    Self: HasMathExprType<MathExpr = MathExpr>
        + HasLispExprType<LispExpr = LispExpr>
        + CanComputeRef<Code, MathExpr, Output = LispExpr>,
    MathSubExpr: BinarySubExpression<MathExpr>,
    Operator: Default + Display,
    LispSubExpr<LispExpr>: CanUpcast<LispExpr>,
{
    type Output = LispExpr;

    fn compute_ref(&self, code: PhantomData<Code>, expr: &MathSubExpr) -> Self::Output {
        let expr_a = self.compute_ref(code, expr.left());
        let expr_b = self.compute_ref(code, expr.right());
        let ident = LispSubExpr::Ident(Ident(Operator::default().to_string())).upcast(PhantomData);

        LispSubExpr::List(List(vec![ident.into(), expr_a.into(), expr_b.into()])).upcast(PhantomData)
    }
}
```

`BinaryOpToLisp<Operator>` works for any `MathSubExpr` whose `left` and `right` fields the [`#[cgp_auto_getter]`](../reference/macros/cgp_auto_getter.md) trait `BinarySubExpression` can read — which is why `Plus` and `Times` derived `HasField` earlier. Both operators now wire to the same provider with a different [`Symbol!`](../reference/macros/symbol.md) operator string:

```rust
ComputerRefComponent:
    UseInputDelegate<new ToLispComponents {
        MathExpr: DispatchToLisp,
        Literal<Value>: LiteralToLisp,
        Plus<MathExpr>: BinaryOpToLisp<Symbol!("+")>,
        Times<MathExpr>: BinaryOpToLisp<Symbol!("*")>,
    }>,
```

## Dispatching on the operation as well as the input

Evaluation and conversion can share one component by adding a second layer of dispatch keyed on the *operation*. Marker types name the operations, and a per-variant provider routes on them with [`UseDelegate`](../reference/providers/use_delegate.md):

```rust
pub struct Eval;
pub struct ToLisp;

delegate_components! {
    new HandlePlus {
        ComputerRefComponent: UseDelegate<new PlusHandlers {
            Eval: EvalAdd,
            ToLisp: BinaryOpToLisp<Symbol!("+")>,
        }>
    }
}
```

`HandlePlus` interprets a `Plus` as either evaluation or conversion depending on the `Code`, and `HandleTimes`, `HandleLiteral`, and `HandleMathExpr` follow the same shape. The context then dispatches first on the input type and second on the operation — a two-layer table whose order is a free choice that costs nothing at runtime, since all of it resolves through trait selection at compile time. Defining the operation routing through `delegate_components!` rather than separate `impl` blocks is what keeps the `Eval` and `ToLisp` logic free to live in different crates.

## Extending the language

A new language lives alongside the old one rather than replacing it, which is the whole point of keeping variants standalone. Subtraction and negation get their own types and evaluation providers, written exactly like the originals:

```rust
pub struct Minus<Expr> { pub left: Box<Expr>, pub right: Box<Expr> }
pub struct Negate<Expr>(pub Box<Expr>);

#[cgp_impl(new EvalSubtract)]
impl<Code, MathExpr, Output> ComputerRef<Code, Minus<MathExpr>>
where
    Self: CanComputeRef<Code, MathExpr, Output = Output>,
    Output: Sub<Output = Output>,
{
    type Output = Output;

    fn compute_ref(&self, code: PhantomData<Code>, Minus { left, right }: &Minus<MathExpr>) -> Self::Output {
        let output_a = self.compute_ref(code, left);
        let output_b = self.compute_ref(code, right);
        output_a - output_b
    }
}
```

The extended enum reuses the original operator providers, now instantiated at a signed `Value` so negation has something to return:

```rust
pub type Value = i64;

#[derive(Debug, HasFields, FromVariant, ExtractField)]
pub enum MathPlusExpr {
    Plus(Plus<MathPlusExpr>),
    Times(Times<MathPlusExpr>),
    Literal(Literal<Value>),
    Negate(Negate<MathPlusExpr>),
    Minus(Minus<MathPlusExpr>),
}

delegate_components! {
    InterpreterPlus {
        ComputerRefComponent:
            UseDelegate<new CodeComponents {
                Eval: UseInputDelegate<new EvalComponents {
                    MathPlusExpr: DispatchEval,
                    Plus<MathPlusExpr>: EvalAdd,
                    Times<MathPlusExpr>: EvalMultiply,
                    Literal<Value>: EvalLiteral,
                    Minus<MathPlusExpr>: EvalSubtract,
                    Negate<MathPlusExpr>: EvalNegate,
                }>,
            }>
    }
}
```

`EvalAdd` and `EvalMultiply` work unchanged because `i64` implements `Add` and `Mul` just as `u64` did — the providers never named a concrete numeric type. Equally telling is what is *absent*: `InterpreterPlus` wires only evaluation and simply omits a to-Lisp handler for `Minus` and `Negate`. Because CGP wiring is lazy and checked only where it is used, the evaluator compiles and runs without those handlers, so a new variant can be prototyped against one operation before the others catch up — the kind of partial extension a closed `enum` with exhaustive `match`es cannot express.
