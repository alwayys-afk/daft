use daft::{Diffable, DiffableOwned, Leaf};
use std::{
    collections::{BTreeMap, BTreeSet},
    fmt::Debug,
};
use uuid::Uuid;

#[derive(Debug, Eq, PartialEq, Diffable)]
enum SomeEnum {
    A,
    B,
    C(u32),
}

#[derive(Debug, Eq, PartialEq, Diffable)]
struct SomeStruct {
    a: i32,
}

#[derive(Debug, Eq, PartialEq, Diffable)]
struct Large {
    a: i32,
    b: SomeEnum,
    c: BTreeMap<Uuid, BTreeSet<usize>>,
    d: SomeStruct,
}

#[derive(Debug, Eq, PartialEq, Diffable)]
struct TupleStruct(String);

#[test]
fn test_basic() {
    let a = SomeEnum::A;
    let b = SomeEnum::B;

    // Enums are just `Leaf`s. We don't try to walk them. User code can do that
    // as necessary.
    let diff = a.diff(&b);
    let expected = Leaf { before: &SomeEnum::A, after: &SomeEnum::B };
    assert_eq!(diff, expected);

    let a = SomeStruct { a: 0 };
    let b = SomeStruct { a: 1 };
    let diff = a.diff(&b);
    let expected = SomeStructDiff { a: Leaf { before: &0, after: &1 } };
    assert_eq!(diff, expected);

    let shared_id = Uuid::new_v4();
    let c1: BTreeMap<Uuid, BTreeSet<usize>> =
        [(shared_id, [1, 2, 3, 4, 5].into_iter().collect())]
            .into_iter()
            .collect();
    let mut c2 = c1.clone();
    c2.get_mut(&shared_id).unwrap().remove(&3);
    c2.get_mut(&shared_id).unwrap().insert(6);
    c2.insert(Uuid::new_v4(), [9].into_iter().collect());
    let a = Large { a: 0, b: SomeEnum::C(4), c: c1, d: SomeStruct { a: 0 } };
    let b = Large { a: 0, b: SomeEnum::B, c: c2, d: SomeStruct { a: 1 } };
    let diff = a.diff(&b);
    println!("{diff:#?}");

    assert_eq!(diff.a.before, diff.a.after);
    assert_eq!(diff.b.before, &SomeEnum::C(4));
    assert_eq!(diff.b.after, &SomeEnum::B);
    assert_eq!(diff.c.unchanged().count(), 0);
    assert_eq!(diff.c.added.len(), 1);
    assert_eq!(diff.c.removed.len(), 0);
    assert_eq!(diff.c.modified().count(), 1);

    let set_diff = &diff.c.modified_diff().next().unwrap().1;
    assert_eq!(set_diff.common, [&1, &2, &4, &5].into_iter().collect());
    assert_eq!(set_diff.added, [&6].into_iter().collect());
    assert_eq!(set_diff.removed, [&3].into_iter().collect());

    assert_eq!(diff.d.a.before, &0);
    assert_eq!(diff.d.a.after, &1);

    let a = TupleStruct("oxide".into());
    let b = TupleStruct("computer company".into());
    let diff = a.diff(&b);
    assert_eq!(diff.0.before, &"oxide".to_string());
    assert_eq!(diff.0.after, &"computer company".to_string());
    println!("{diff:#?}");
}

#[test]
fn test_enum_with_generics() {
    #[derive(Debug, Eq, PartialEq, Diffable)]
    enum EnumWithGenerics<'a, T, U> {
        A(T),
        B(&'a U),
    }

    let x = 5usize;
    let y = 5u8;
    let a = EnumWithGenerics::A(x);
    let b = EnumWithGenerics::B(&y);
    let diff = a.diff(&b);
    assert_eq!(Leaf { before: &a, after: &b }, diff);
}

#[test]
fn test_struct_with_generics() {
    #[derive(Debug, Eq, PartialEq, Diffable)]
    struct StructWithGenerics<'d, 'e, T, U>
    where
        T: Diffable + 'd,
        U: Diffable + 'e,
    {
        b: usize,
        c: &'d T,
        d: &'e U,
    }

    let x = StructWithGenerics { b: 6, c: &5, d: &6 };
    let y = StructWithGenerics { b: 7, c: &5, d: &7 };
    let diff = x.diff(&y);

    assert_eq!(diff.b, Leaf { before: &6, after: &7 });
    assert_eq!(diff.c, Leaf { before: &5, after: &5 });
    assert_eq!(diff.d, Leaf { before: &6, after: &7 });
    println!("{diff:?}");

    #[derive(Debug, Eq, PartialEq, Diffable)]
    struct S<'a, T, U>
    where
        T: Diffable + Eq + 'a,
        U: Diffable + 'a,
    {
        a: BTreeMap<usize, T>,
        b: usize,
        c: &'a U,
        d: &'a str,
    }

    let x = S {
        a: [(5, 2usize)].into_iter().collect(),
        b: 5,
        c: &6usize,
        d: "hello",
    };
    let y = S {
        a: [(5, 1usize)].into_iter().collect(),
        b: 5,
        c: &6usize,
        d: "world",
    };
    let diff = x.diff(&y);

    assert_eq!(diff.a.unchanged().count(), 0);
    assert_eq!(diff.a.modified().count(), 1);
    assert_eq!(diff.a.added.len(), 0);
    assert_eq!(diff.a.removed.len(), 0);
    assert_eq!(diff.b.before, diff.b.after);
    assert_eq!(diff.c.before, diff.c.after);
    assert_eq!(diff.d.before, "hello");
    assert_eq!(diff.d.after, "world");

    println!("{diff:#?}");
}

#[test]
fn diff_pair_lifetimes() {
    // Complex type to ensure lifetimes are correct.
    #[derive(Diffable)]
    struct Inner {
        a: u32,
        b: &'static str,
    }

    #[derive(Diffable)]
    struct Outer {
        #[daft(leaf)]
        inner: Inner,
    }

    let owned: Leaf<String> = {
        let before = Outer { inner: Inner { a: 5, b: "hello" } };
        let after = Outer { inner: Inner { a: 6, b: "world" } };

        let diff = before.diff(&after);
        let inner_diff = {
            let inner: Leaf<&Inner> = diff.inner;
            // Ensure that inner.diff_pair outlives inner.
            inner.diff_pair()
        };

        assert_eq!(*inner_diff.a.before, 5);
        assert_eq!(*inner_diff.a.after, 6);
        assert_eq!(inner_diff.b.before, "hello");
        assert_eq!(inner_diff.b.after, "world");

        // The return value of this will outlive before and after as well.
        inner_diff.b.map(str::to_owned)
    };

    assert_eq!(owned.before, "hello");
    assert_eq!(owned.after, "world");
}

// ---------------------------------------------------------------------------
// DiffableOwned tests
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Eq, PartialEq, DiffableOwned, serde::Serialize, serde::Deserialize)]
enum OwnedEnum {
    A,
    B,
    C(u32),
}

#[derive(Debug, Clone, Eq, PartialEq, DiffableOwned, serde::Serialize, serde::Deserialize)]
struct OwnedSimple {
    a: i32,
}

#[derive(Debug, Clone, Eq, PartialEq, DiffableOwned, serde::Serialize, serde::Deserialize)]
struct OwnedLarge {
    a: i32,
    b: OwnedEnum,
    c: BTreeMap<String, BTreeSet<usize>>,
    d: OwnedSimple,
}

#[derive(Debug, Eq, PartialEq, DiffableOwned)]
struct OwnedTupleStruct(String);

#[test]
fn test_owned_basic() {
    // Enum: produces Leaf<Self>
    let diff = OwnedEnum::A.diff_owned(OwnedEnum::B);
    assert_eq!(diff, Leaf { before: OwnedEnum::A, after: OwnedEnum::B });

    // Struct: sparse — changed fields are Some, unchanged are None
    let diff = OwnedSimple { a: 0 }.diff_owned(OwnedSimple { a: 1 });
    assert_eq!(diff, OwnedSimpleDiffOwned { a: Some(Leaf { before: 0, after: 1 }) });

    // Unchanged fields are None
    let diff = OwnedSimple { a: 5 }.diff_owned(OwnedSimple { a: 5 });
    assert_eq!(diff, OwnedSimpleDiffOwned { a: None });

    // Tuple struct
    let diff = OwnedTupleStruct("hello".into())
        .diff_owned(OwnedTupleStruct("world".into()));
    assert_eq!(
        diff.0,
        Some(Leaf { before: "hello".to_owned(), after: "world".to_owned() })
    );
}

#[test]
fn test_owned_complex() {
    let c1: BTreeMap<String, BTreeSet<usize>> =
        [("key".into(), [1, 2, 3].into_iter().collect())]
            .into_iter()
            .collect();
    let mut c2 = c1.clone();
    c2.get_mut("key").unwrap().remove(&2);
    c2.get_mut("key").unwrap().insert(4);
    c2.insert("new".into(), [9].into_iter().collect());

    let a = OwnedLarge {
        a: 0,
        b: OwnedEnum::C(4),
        c: c1,
        d: OwnedSimple { a: 0 },
    };
    let b = OwnedLarge {
        a: 0,
        b: OwnedEnum::B,
        c: c2,
        d: OwnedSimple { a: 1 },
    };
    let diff = a.diff_owned(b);
    println!("{diff:#?}");

    // a is unchanged — None
    assert!(diff.a.is_none());
    // b changed
    let b_diff = diff.b.unwrap();
    assert_eq!(b_diff.before, OwnedEnum::C(4));
    assert_eq!(b_diff.after, OwnedEnum::B);
    // map diff
    let c_diff = diff.c.unwrap();
    assert_eq!(c_diff.added.len(), 1);
    assert!(c_diff.added.contains_key("new"));
    assert_eq!(c_diff.removed.len(), 0);
    assert_eq!(c_diff.common.len(), 1);
    // nested struct diff
    let d_diff = diff.d.unwrap();
    assert_eq!(d_diff.a.unwrap().before, 0);
    assert_eq!(d_diff.a.unwrap().after, 1);
}

#[test]
fn test_owned_outlives_originals() {
    // The key advantage: the diff outlives the originals.
    let diff = {
        let a = OwnedSimple { a: 42 };
        let b = OwnedSimple { a: 99 };
        a.diff_owned(b)
        // a and b are consumed here
    };

    // diff is fully owned - still valid
    let a_diff = diff.a.unwrap();
    assert_eq!(a_diff.before, 42);
    assert_eq!(a_diff.after, 99);
}

#[test]
fn test_owned_with_attributes() {
    #[derive(Debug, Eq, PartialEq, DiffableOwned, serde::Serialize, serde::Deserialize)]
    struct Inner {
        x: usize,
    }

    #[derive(Debug, Eq, PartialEq, serde::Serialize, serde::Deserialize)]
    struct NotDiffable(usize);

    #[derive(Debug, Eq, PartialEq, DiffableOwned)]
    struct WithAttrs {
        a: i32,
        #[daft(ignore)]
        _b: String,
        #[daft(leaf)]
        c: Inner,
        #[daft(leaf)]
        d: NotDiffable,
    }

    let a = WithAttrs {
        a: 1,
        _b: "ignored".into(),
        c: Inner { x: 10 },
        d: NotDiffable(1),
    };
    let b = WithAttrs {
        a: 2,
        _b: "also ignored".into(),
        c: Inner { x: 20 },
        d: NotDiffable(2),
    };
    let diff = a.diff_owned(b);

    assert_eq!(diff.a, Some(Leaf { before: 1, after: 2 }));
    // _b is ignored, not present in diff
    // c is a leaf (not recursively diffed)
    assert_eq!(
        diff.c,
        Some(Leaf { before: Inner { x: 10 }, after: Inner { x: 20 } })
    );
    assert_eq!(
        diff.d,
        Some(Leaf { before: NotDiffable(1), after: NotDiffable(2) })
    );
}

#[test]
fn test_owned_primitives() {
    // Test that primitive types work with DiffableOwned.
    assert_eq!(42i32.diff_owned(99), Leaf { before: 42, after: 99 });
    assert_eq!(true.diff_owned(false), Leaf { before: true, after: false });
    assert_eq!(
        "hello".to_owned().diff_owned("world".to_owned()),
        Leaf { before: "hello".to_owned(), after: "world".to_owned() }
    );
}

#[test]
fn test_owned_collections() {
    // BTreeMap
    let a: BTreeMap<i32, &str> = [(1, "a"), (2, "b")].into_iter().collect();
    let b: BTreeMap<i32, &str> = [(2, "B"), (3, "c")].into_iter().collect();
    let diff = a.diff_owned(b);

    assert_eq!(diff.removed, [(1, "a")].into_iter().collect());
    assert_eq!(diff.added, [(3, "c")].into_iter().collect());
    assert_eq!(diff.common.len(), 1);
    assert_eq!(
        diff.common[&2],
        Leaf { before: "b", after: "B" }
    );

    // BTreeSet
    let a: BTreeSet<i32> = [1, 2, 3].into_iter().collect();
    let b: BTreeSet<i32> = [2, 3, 4].into_iter().collect();
    let diff = a.diff_owned(b);

    assert_eq!(diff.common, [2, 3].into_iter().collect());
    assert_eq!(diff.added, [4].into_iter().collect());
    assert_eq!(diff.removed, [1].into_iter().collect());
}

#[test]
fn test_owned_tuples() {
    let diff = (1i32, "hello".to_owned()).diff_owned((2, "world".to_owned()));
    assert_eq!(diff.0, Leaf { before: 1, after: 2 });
    assert_eq!(
        diff.1,
        Leaf { before: "hello".to_owned(), after: "world".to_owned() }
    );
}

#[test]
fn test_owned_wrapper_types() {
    // Box
    let diff = Box::new(42i32).diff_owned(Box::new(99));
    assert_eq!(diff, Leaf { before: 42, after: 99 });

    // Vec (leaf)
    let diff = vec![1, 2, 3].diff_owned(vec![4, 5, 6]);
    assert_eq!(diff, Leaf { before: vec![1, 2, 3], after: vec![4, 5, 6] });

    // Option
    let diff = Some(1i32).diff_owned(Some(2));
    assert_eq!(diff, Leaf { before: Some(1), after: Some(2) });
}

// ---------------------------------------------------------------------------
// Serde round-trip tests
// ---------------------------------------------------------------------------

#[test]
fn test_serde_leaf() {
    let leaf = Leaf { before: 42i32, after: 99 };
    let json = serde_json::to_string(&leaf).unwrap();
    let deserialized: Leaf<i32> = serde_json::from_str(&json).unwrap();
    assert_eq!(leaf, deserialized);
}

#[test]
fn test_serde_owned_map_diff() {
    let a: BTreeMap<String, i32> =
        [("x".into(), 1), ("y".into(), 2)].into_iter().collect();
    let b: BTreeMap<String, i32> =
        [("y".into(), 3), ("z".into(), 4)].into_iter().collect();
    let diff = a.diff_owned(b);

    let json = serde_json::to_string(&diff).unwrap();
    let deserialized: daft::BTreeMapDiffOwned<String, i32> =
        serde_json::from_str(&json).unwrap();
    assert_eq!(diff, deserialized);
}

#[test]
fn test_serde_owned_set_diff() {
    let a: BTreeSet<i32> = [1, 2, 3].into_iter().collect();
    let b: BTreeSet<i32> = [2, 3, 4].into_iter().collect();
    let diff = a.diff_owned(b);

    let json = serde_json::to_string(&diff).unwrap();
    let deserialized: daft::BTreeSetDiffOwned<i32> =
        serde_json::from_str(&json).unwrap();
    assert_eq!(diff, deserialized);
}

#[test]
fn test_serde_leaf_string() {
    let leaf: Leaf<String> =
        "hello".to_owned().diff_owned("world".to_owned());
    let json = serde_json::to_string(&leaf).unwrap();
    let deserialized: Leaf<String> = serde_json::from_str(&json).unwrap();
    assert_eq!(leaf, deserialized);

    let value: serde_json::Value = serde_json::from_str(&json).unwrap();
    assert_eq!(value["before"], "hello");
    assert_eq!(value["after"], "world");
}
