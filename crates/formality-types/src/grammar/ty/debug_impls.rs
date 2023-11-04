use super::{AliasName, AliasTy, AssociatedTyName, Parameter, RefKind, RigidName, RigidTy};
use std::fmt::Debug;

impl Debug for RigidTy {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let RigidTy { name, parameters } = self;
        match name {
            RigidName::AdtId(name) => {
                write!(
                    f,
                    "{:?}{:?}",
                    name,
                    PrettyParameters::new("<", ">", parameters)
                )
            }
            RigidName::ScalarId(s) if parameters.is_empty() => {
                write!(f, "{:?}", s)
            }
            RigidName::Ref(RefKind::Shared) if parameters.len() == 2 => {
                write!(f, "&{:?} {:?}", parameters[0], parameters[1])
            }
            RigidName::Ref(RefKind::Mut) if parameters.len() == 2 => {
                write!(f, "&mut {:?} {:?}", parameters[0], parameters[1])
            }
            RigidName::Tuple(arity) if parameters.len() == *arity => {
                if *arity != 0 {
                    write!(f, "{:?}", PrettyParameters::new("(", ")", parameters))
                } else {
                    // PrettyParameters would skip the separators
                    // for 0 arity
                    write!(f, "()")
                }
            }
            _ => {
                write!(
                    f,
                    "{:?}{:?}",
                    name,
                    PrettyParameters::new("<", ">", parameters)
                )
            }
        }
    }
}

impl Debug for AliasTy {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let AliasTy { name, parameters } = self;
        match name {
            AliasName::AssociatedTyId(AssociatedTyName { trait_id, item_id }) => {
                // Grr, wish we would remember the number of parameters assigned to each position.
                write!(
                    f,
                    "({:?}::{:?}){:?}",
                    trait_id,
                    item_id,
                    PrettyParameters::new("<", ">", parameters),
                )
            }
        }
    }
}

struct PrettyParameters<'a> {
    open: &'a str,
    close: &'a str,
    p: &'a [Parameter],
}
impl<'a> PrettyParameters<'a> {
    fn new(open: &'a str, close: &'a str, p: &'a [Parameter]) -> Self {
        Self { open, close, p }
    }
}

impl Debug for PrettyParameters<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.p.len() == 0 {
            Ok(())
        } else {
            write!(f, "{}", self.open)?;
            write!(f, "{:?}", self.p[0])?;
            for p in &self.p[1..] {
                write!(f, ", {:?}", p)?;
            }
            write!(f, "{}", self.close)?;
            Ok(())
        }
    }
}
