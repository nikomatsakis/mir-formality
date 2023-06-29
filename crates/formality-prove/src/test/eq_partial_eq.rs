use expect_test::expect;
use formality_macros::test;
use formality_types::{
    grammar::{Wc, Wcs},
    parse::term,
};

use crate::{decls::Decls, prove::prove};

/// Simple example decls consisting only of two trait declarations.
fn decls() -> Decls {
    Decls {
        trait_decls: vec![
            term("trait Eq<ty Self> where {PartialEq(Self)}"),
            term("trait PartialEq<ty Self> where {}"),
        ],
        ..Decls::empty()
    }
}

#[test]
fn eq_implies_partial_eq() {
    let assumptions: Wcs = Wcs::t();
    let goal: Wc = term("for<ty T> if {Eq(T)} PartialEq(T)");
    let constraints = prove(decls(), (), assumptions, goal);
    expect![[r#"
        {
            Constraints {
                env: Env {
                    variables: [],
                    coherence_mode: false,
                },
                known_true: true,
                substitution: {},
            },
        }
    "#]]
    .assert_debug_eq(&constraints);
}

#[test]
fn not_partial_eq_implies_eq() {
    let goal: Wc = term("for<ty T> if {PartialEq(T)} Eq(T)");
    let constraints = prove(decls(), (), (), goal);
    expect![[r#"
        {}
    "#]]
    .assert_debug_eq(&constraints);
}

#[test]
fn universals_not_eq() {
    let goal: Wc = term("for<ty T, ty U> if {Eq(T)} PartialEq(U)");
    let constraints = prove(decls(), (), (), goal);
    expect![[r#"
        {}
    "#]]
    .assert_debug_eq(&constraints);
}
