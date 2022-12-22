use std::collections::BTreeSet;

use formality_macros::term;
use formality_types::grammar::{
    AtomicRelation, Binder, ElaboratedHypotheses, Goal, InferenceVar, Universe,
};

use super::Env;

mod extract_query_result;
mod querify;
mod test;

/// A `Query` is a canonical description of a goal to be proven under a certain set of
/// assumptions, along with a minimal environment in which to prove it.
///
/// The environment contains only the (unbound) inference variables
/// that appear in the assumptions or goal; the numbering of those variables is consistent
/// such that any two queries with same assumptions/goals will have the same numbering
/// (e.g., left-to-right in order of appearance).
///
/// The environment also contains only the universes for placeholders that appear in the
/// assumptions/goals, and those universes are compresssed so they don't contain any
/// gaps.
#[term]
pub struct Query {
    pub env: Env,
    pub assumptions: ElaboratedHypotheses,
    pub goal: Goal,
}

impl Query {
    pub fn query_variables(&self) -> Vec<InferenceVar> {
        self.env.inference_variables()
    }
}

#[term]
pub struct UniverseMap {
    pub universes: Vec<(Universe, Universe)>,
}

#[term]
pub struct QueryResult {
    /// "Forall" variables in the query result. These are to be
    /// instantiated as existentials.
    pub binder: Binder<QueryResultBoundData>,
}

#[term]
pub struct QueryResultBoundData {
    /// Non-equality relations between inference variables in the initial environment
    /// or fresh variables. For example, `a: b`.
    pub relations: Vec<AtomicRelation>,
}

pub use querify::querify;

/// Helper: Remove duplicates from `vec`, preserving the ordering.
fn dedup<T: Clone + Ord>(vec: &mut Vec<T>) {
    let mut set: BTreeSet<T> = BTreeSet::default();
    vec.retain(|e| set.insert(e.clone()));
}
