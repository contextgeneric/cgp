# Extracting CGP compile errors

How to turn a CGP compile error — often a wall of repeated failures naming generated types — into a compact, root-cause-first summary. This skill serves two roles, and which one applies depends on your task: an agent **extracting** an error reads raw compiler output and reduces it to a few facts, while an agent **delegating** hands a long error to a sub-agent so it never touches the main context. Both roles rely on the same shape knowledge, so read [The two shapes](#the-two-shapes-to-recognize) first, then jump to [Role 1 — extracting](#role-1--extracting-an-error-yourself) or [Role 2 — delegating](#role-2--delegating-the-extraction-to-a-sub-agent).

## Why extraction is a skill of its own

CGP error output is disproportionately large and disproportionately misleading, so reading it well is a distinct task from fixing the code. Wiring is resolved [lazily](checking.md), so one broken link surfaces at every place that transitively needs it, and each failure quotes *generated* code — `IsProviderFor`, `DelegateComponent`, `CanUseComponent`, and the type-level `Symbol`/`Chars`/`PathCons` spines you never wrote. A single missing field can print screens of near-identical errors. Worse, some of that output is actively deceptive: a whole class of CGP errors reports that a trait's bounds are unsatisfied while *hiding* the dependency that actually failed. Reading such output inline, in the agent trying to fix the code, wastes context on noise and risks chasing a cause the output does not contain.

## The two shapes to recognize

Before decoding any nested type, read the *trait* in the error and decide which of two shapes you are looking at, because the shapes differ in whether the root cause is even present. The [macro-grammar](macro-grammar.md) decoder and the [checking](checking.md) playbook cover the full set; extraction needs the one binary distinction.

A **surfaced** error carries the root cause. It is an `E0277` note chain, typically topped by `CanUseComponent` or `IsProviderFor`, that names a concrete missing bound such as `HasField<Symbol!("name")>`. This is what a [`check_components!`](checking.md) assertion produces, because the check requires `IsProviderFor` as a *direct* bound and forces the compiler to evaluate the provider's `where` clause. The concrete bound is the fact to extract — the compiler names it in a `help:`/"is not implemented" note, and the `required for …` notes trace the dependency path from it back to the check.

A **hidden** error does not. It is an `E0599` "the method `greet` exists for struct `Person`, but its trait bounds were not satisfied", or an `E0277` that a consumer trait like `Person: CanGreet` is unsatisfied, and it names the consumer or provider trait and then stops — with no note descending to the missing field or dependency. This happens when broken wiring is exercised by *calling the consumer-trait method directly* rather than through a check: the compiler sees the consumer trait's blanket impl among the candidate impls, finds it inapplicable, and suppresses the nested bound that made it so. The cause is **absent**, not buried. Do not scan a hidden error for a root cause; there is none to find. The fix is to *promote* it into a surfaced error — add a `check_components!` for the failing component at the wiring site — and read that instead.

## Role 1 — extracting an error yourself

You are in this role whenever you are the one reading the raw output — as a sub-agent handed the job, or as the main agent facing an error short enough to read inline. Your product is the [compact summary](#reduce-to-the-compact-summary) below, never the raw dump.

### Capture the output without flooding context

Capture the compiler output to a file rather than letting it stream into your transcript, and target the smallest unit that reproduces the failure. A whole-workspace build multiplies the cascade across crates; a single crate, test, example, or ten-line scratch module shrinks it to the one thing you care about. The commands are just your project's ordinary build and test invocations — nothing CGP-specific:

```bash
# Target the smallest failing unit and redirect everything to a scratch file.
cargo check -p <your-crate> 2>&1 | tee /tmp/cgp-error.txt

# or a single test / example that exercises the failing wiring:
cargo test -p <your-crate> --test <target> 2>&1 | tee /tmp/cgp-error.txt
```

Two details recur, both plain `rustc` behavior rather than anything CGP-specific. When a type in the error is elided as `...`, the compiler writes its full form to a file named in a final note (`the full name for the type has been written to '….long-type-….txt'`); that elided middle is frequently the one segment that reveals *which* context or path the error is about, so read that file when the cause hinges on a long type. And watch for the **near-contradiction** shape — "the trait `X` is not implemented for `T`" immediately followed by a `help:` note that `X` *is* implemented for `T` — which means an impl exists but a nested bound it carries does not hold, or two candidates are ambiguous; trust the error, not the `help:`.

When the cause hinges on *what the macro emitted* rather than on the message itself, stop reading the error and read the expansion. `cargo expand` prints the macro-expanded source in any project; in a project set up with the CGP test utilities, the `snapshot_*!` helpers from `cgp-macro-test-util` additionally pin an expansion as a reviewable snapshot. The [macro-grammar](macro-grammar.md) skill covers how to read the expanded impls once you have them.

### Reduce to the compact summary

Reduce the output to the same few facts — the anatomy a CGP error catalog records — and nothing more:

- **Class and code** — the error code(s) and the trait(s) named (`E0599` on a provider trait, `E0277` through `CanUseComponent`, `E0119` conflicting `DelegateComponent`, `E0207` unconstrained generic, and so on).
- **Hidden or surfaced** — whether the root cause is present in the output at all.
- **Root cause and position** — if surfaced, the concrete failing bound and where it sits (in the compiler's `help:` note, near the last or second-to-last block of a cascade, or inside the `long-type-….txt` file); if hidden, say so plainly.
- **Recommended next action** — fix the named field or wiring, promote a hidden error with a `check_components!`, break a cycle, remove a duplicate key — whatever the class implies.

A good summary is a few lines, not a transcript. Never hand back the raw output; that defeats the purpose of extracting it.

## Role 2 — delegating the extraction to a sub-agent

You are in this role when you are the main agent and the error is long — a deep cascade, many crates, or output that runs to screens. Spawn a sub-agent to read it and return only the [compact summary](#reduce-to-the-compact-summary); the sub-agent absorbs the wall of text while you keep your own context clean. Give the sub-agent three things: the exact command to run (or the path to an already-captured output file), this skill so it knows the two shapes and the summary format, and the instruction to return *only* the summary. This is the intended workflow both when documenting an error class — where a sub-agent gathers the facts from a failing reproduction — and in an ordinary debugging session, where a sub-agent reads an error too long to justify reading inline. When the returned summary says the error is hidden, the follow-up is almost always to add a check and re-run, which itself may be worth delegating.

## Further reference

- Sibling skills: [checking](checking.md) for why wiring is lazy and how checks force a surfaced error; [macro-grammar](macro-grammar.md) for the full error decoder and how to read an expansion; [wiring](wiring.md) for the delegation mechanics behind a conflict or cycle.
- Online: the [error catalog](https://github.com/contextgeneric/cgp/tree/main/docs/errors) documents each error class and where its root cause sits, and the [debugging guide](https://github.com/contextgeneric/cgp/blob/main/docs/guides/debugging.md) is the full tracing playbook.
