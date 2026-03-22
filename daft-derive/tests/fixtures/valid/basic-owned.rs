use daft::DiffableOwned;
use std::collections::{BTreeMap, BTreeSet};
use uuid::Uuid;

#[derive(Debug, Eq, PartialEq, DiffableOwned)]
struct BasicOwned {
    a: i32,
    b: BTreeMap<Uuid, BTreeSet<usize>>,
}

fn main() {}
