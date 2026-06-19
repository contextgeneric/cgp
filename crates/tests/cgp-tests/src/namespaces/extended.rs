use cgp::prelude::DefaultNamespace;
use cgp_macro_test_util::snapshot_cgp_namespace;

snapshot_cgp_namespace! {
    cgp_namespace! {
        new ExtendedNamespace: DefaultNamespace {
            @cgp.core.error =>
                @app,
        }
    }

    expand_extended_namespace(output) {
        insta::assert_snapshot!(output, @"
        pub struct __ExtendedNamespaceComponents;
        pub trait ExtendedNamespace<__Table__> {
            type Delegate;
        }
        impl<__Table__, __Key__, __Value__> ExtendedNamespace<__Table__> for __Key__
        where
            __Key__: DefaultNamespace<__ExtendedNamespaceComponents>,
            __Key__: DefaultNamespace<__Table__, Delegate = __Value__>,
        {
            type Delegate = __Value__;
        }
        impl<__Table__, __Wildcard__> ExtendedNamespace<__Table__>
        for PathCons<
            Symbol<3, Chars<'c', Chars<'g', Chars<'p', Nil>>>>,
            PathCons<
                Symbol<4, Chars<'c', Chars<'o', Chars<'r', Chars<'e', Nil>>>>>,
                PathCons<
                    Symbol<5, Chars<'e', Chars<'r', Chars<'r', Chars<'o', Chars<'r', Nil>>>>>>,
                    __Wildcard__,
                >,
            >,
        > {
            type Delegate = RedirectLookup<
                __Table__,
                PathCons<Symbol<3, Chars<'a', Chars<'p', Chars<'p', Nil>>>>, __Wildcard__>,
            >;
        }
        ")
    }
}
