struct BasicOwnedDiffOwned {
    a: Option<<i32 as ::daft::DiffableOwned>::DiffOwned>,
    b: Option<<BTreeMap<Uuid, BTreeSet<usize>> as ::daft::DiffableOwned>::DiffOwned>,
}
impl ::core::fmt::Debug for BasicOwnedDiffOwned
where
    Option<<i32 as ::daft::DiffableOwned>::DiffOwned>: ::core::fmt::Debug,
    Option<
        <BTreeMap<Uuid, BTreeSet<usize>> as ::daft::DiffableOwned>::DiffOwned,
    >: ::core::fmt::Debug,
{
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct(stringify!(BasicOwnedDiffOwned))
            .field(stringify!(a), &self.a)
            .field(stringify!(b), &self.b)
            .finish()
    }
}
impl ::core::cmp::PartialEq for BasicOwnedDiffOwned
where
    Option<<i32 as ::daft::DiffableOwned>::DiffOwned>: ::core::cmp::PartialEq,
    Option<
        <BTreeMap<Uuid, BTreeSet<usize>> as ::daft::DiffableOwned>::DiffOwned,
    >: ::core::cmp::PartialEq,
{
    fn eq(&self, other: &Self) -> bool {
        self.a == other.a && self.b == other.b
    }
}
impl ::core::cmp::Eq for BasicOwnedDiffOwned
where
    Option<<i32 as ::daft::DiffableOwned>::DiffOwned>: ::core::cmp::Eq,
    Option<
        <BTreeMap<Uuid, BTreeSet<usize>> as ::daft::DiffableOwned>::DiffOwned,
    >: ::core::cmp::Eq,
{}
impl ::daft::DiffableOwned for BasicOwned {
    type DiffOwned = BasicOwnedDiffOwned;
    fn diff_owned(self, other: Self) -> BasicOwnedDiffOwned {
        Self::DiffOwned {
            a: if self.a == other.a {
                None
            } else {
                Some(::daft::DiffableOwned::diff_owned(self.a, other.a))
            },
            b: if self.b == other.b {
                None
            } else {
                Some(::daft::DiffableOwned::diff_owned(self.b, other.b))
            },
        }
    }
}
