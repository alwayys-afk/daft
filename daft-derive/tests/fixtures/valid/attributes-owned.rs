use daft::DiffableOwned;
use std::collections::{BTreeMap, BTreeSet};

#[derive(Debug, Eq, PartialEq, DiffableOwned)]
struct WithAttrsOwned {
    a: i32,
    b: BTreeMap<String, BTreeSet<usize>>,
    #[daft(ignore)]
    c: std::time::Instant,
    #[daft(leaf)]
    d: Lazy,
    #[daft(leaf)]
    e: usize,
    f: usize,
}

#[derive(Debug, Eq, PartialEq, DiffableOwned, serde::Serialize, serde::Deserialize)]
struct Lazy {
    x: usize,
    y: usize,
}

fn main() {}
