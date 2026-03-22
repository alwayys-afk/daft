use daft::DiffableOwned;
use std::collections::{BTreeMap, BTreeSet};

#[derive(Debug, Eq, PartialEq, DiffableOwned)]
struct BasicOwned {
    a: i32,
    b: BTreeMap<String, BTreeSet<usize>>,
}

fn main() {}
