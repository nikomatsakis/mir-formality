use crate::grammar::Const;

impl std::fmt::Debug for super::Ty {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.data() {
            super::TyData::RigidTy(r) => write!(f, "{r:?}"),
            super::TyData::AliasTy(r) => write!(f, "{r:?}"),
            super::TyData::PredicateTy(r) => write!(f, "{r:?}"),
            super::TyData::Variable(r) => write!(f, "{r:?}"),
        }
    }
}

impl std::fmt::Debug for Const {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.data() {
            crate::grammar::ConstData::Value(valtree) => write!(f, "{valtree:?}"),
            crate::grammar::ConstData::Variable(_) => todo!(),
        }
    }
}

impl std::fmt::Debug for super::Lt {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.data() {
            super::LtData::Static => write!(f, "static"),
            super::LtData::Variable(v) => write!(f, "{:?}", v),
        }
    }
}

impl std::fmt::Debug for super::Variable {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::UniversalVar(arg0) => write!(f, "{:?}", arg0),
            Self::ExistentialVar(arg0) => write!(f, "{:?}", arg0),
            Self::BoundVar(arg0) => write!(f, "{:?}", arg0),
        }
    }
}

impl std::fmt::Debug for super::UniversalVar {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let super::UniversalVar { var_index, kind } = self;
        write!(f, "!{:?}_{:?}", kind, var_index)
    }
}

impl std::fmt::Debug for super::ExistentialVar {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let super::ExistentialVar { var_index, kind } = self;
        write!(f, "?{:?}_{:?}", kind, var_index)
    }
}

impl std::fmt::Debug for super::VarIndex {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.index)
    }
}

impl std::fmt::Debug for super::BoundVar {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            super::BoundVar {
                debruijn: None,
                var_index,
                kind,
            } => write!(f, "^{:?}_{:?}", kind, var_index),
            super::BoundVar {
                debruijn: Some(db),
                var_index,
                kind,
            } => write!(f, "^{:?}{:?}_{:?}", kind, db.index, var_index),
        }
    }
}

impl std::fmt::Debug for super::DebruijnIndex {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "^{}", self.index)
    }
}
