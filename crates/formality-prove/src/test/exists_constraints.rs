use expect_test::expect;
use formality_macros::test;
use formality_types::parse::term;

use crate::decls::Decls;

use crate::test_util::test_prove;

/// Simple example decls consisting only of two trait declarations.
fn decls() -> Decls {
    Decls {
        trait_decls: vec![term("safe trait Foo<ty Self> where {}")],
        impl_decls: vec![term("safe impl<ty T> Foo(Vec<T>) where {}")],
        ..Decls::empty()
    }
}

/// Test that `exists<T> Foo(U)` yields `U = Vec<X>` for some fresh `X`
#[test]
fn exists_u_for_t() {
    let constraints = test_prove(decls(), term("exists<ty U> {} => {Foo(U)}"));
    expect![[r#"
        {
            Constraints {
                env: Env {
                    variables: [
                        ?ty_2,
                        ?ty_1,
                    ],
                    coherence_mode: false,
                },
                known_true: true,
                substitution: {
                    ?ty_1 => (rigid (adt Vec) ?ty_2),
                },
            },
        }
    "#]]
    .assert_debug_eq(&constraints);
}
