struct WithAttrsOwnedDiffOwned {
    a: <i32 as ::daft::DiffableOwned>::DiffOwned,
    b: <BTreeMap<Uuid, BTreeSet<usize>> as ::daft::DiffableOwned>::DiffOwned,
    d: ::daft::Leaf<Lazy>,
    e: ::daft::Leaf<usize>,
    f: <usize as ::daft::DiffableOwned>::DiffOwned,
}
impl ::core::fmt::Debug for WithAttrsOwnedDiffOwned
where
    <i32 as ::daft::DiffableOwned>::DiffOwned: ::core::fmt::Debug,
    <BTreeMap<
        Uuid,
        BTreeSet<usize>,
    > as ::daft::DiffableOwned>::DiffOwned: ::core::fmt::Debug,
    ::daft::Leaf<Lazy>: ::core::fmt::Debug,
    ::daft::Leaf<usize>: ::core::fmt::Debug,
    <usize as ::daft::DiffableOwned>::DiffOwned: ::core::fmt::Debug,
{
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct(stringify!(WithAttrsOwnedDiffOwned))
            .field(stringify!(a), &self.a)
            .field(stringify!(b), &self.b)
            .field(stringify!(d), &self.d)
            .field(stringify!(e), &self.e)
            .field(stringify!(f), &self.f)
            .finish()
    }
}
impl ::core::cmp::PartialEq for WithAttrsOwnedDiffOwned
where
    <i32 as ::daft::DiffableOwned>::DiffOwned: ::core::cmp::PartialEq,
    <BTreeMap<
        Uuid,
        BTreeSet<usize>,
    > as ::daft::DiffableOwned>::DiffOwned: ::core::cmp::PartialEq,
    ::daft::Leaf<Lazy>: ::core::cmp::PartialEq,
    ::daft::Leaf<usize>: ::core::cmp::PartialEq,
    <usize as ::daft::DiffableOwned>::DiffOwned: ::core::cmp::PartialEq,
{
    fn eq(&self, other: &Self) -> bool {
        self.a == other.a && self.b == other.b && self.d == other.d && self.e == other.e
            && self.f == other.f
    }
}
impl ::core::cmp::Eq for WithAttrsOwnedDiffOwned
where
    <i32 as ::daft::DiffableOwned>::DiffOwned: ::core::cmp::Eq,
    <BTreeMap<
        Uuid,
        BTreeSet<usize>,
    > as ::daft::DiffableOwned>::DiffOwned: ::core::cmp::Eq,
    ::daft::Leaf<Lazy>: ::core::cmp::Eq,
    ::daft::Leaf<usize>: ::core::cmp::Eq,
    <usize as ::daft::DiffableOwned>::DiffOwned: ::core::cmp::Eq,
{}
impl ::daft::DiffableOwned for WithAttrsOwned {
    type DiffOwned = WithAttrsOwnedDiffOwned;
    fn diff_owned(self, other: Self) -> WithAttrsOwnedDiffOwned {
        Self::DiffOwned {
            a: ::daft::DiffableOwned::diff_owned(self.a, other.a),
            b: ::daft::DiffableOwned::diff_owned(self.b, other.b),
            d: ::daft::Leaf {
                before: self.d,
                after: other.d,
            },
            e: ::daft::Leaf {
                before: self.e,
                after: other.e,
            },
            f: ::daft::DiffableOwned::diff_owned(self.f, other.f),
        }
    }
}
struct LazyDiffOwned {
    x: <usize as ::daft::DiffableOwned>::DiffOwned,
    y: <usize as ::daft::DiffableOwned>::DiffOwned,
}
impl ::core::fmt::Debug for LazyDiffOwned
where
    <usize as ::daft::DiffableOwned>::DiffOwned: ::core::fmt::Debug,
    <usize as ::daft::DiffableOwned>::DiffOwned: ::core::fmt::Debug,
{
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct(stringify!(LazyDiffOwned))
            .field(stringify!(x), &self.x)
            .field(stringify!(y), &self.y)
            .finish()
    }
}
impl ::core::cmp::PartialEq for LazyDiffOwned
where
    <usize as ::daft::DiffableOwned>::DiffOwned: ::core::cmp::PartialEq,
    <usize as ::daft::DiffableOwned>::DiffOwned: ::core::cmp::PartialEq,
{
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y
    }
}
impl ::core::cmp::Eq for LazyDiffOwned
where
    <usize as ::daft::DiffableOwned>::DiffOwned: ::core::cmp::Eq,
    <usize as ::daft::DiffableOwned>::DiffOwned: ::core::cmp::Eq,
{}
impl ::daft::DiffableOwned for Lazy {
    type DiffOwned = LazyDiffOwned;
    fn diff_owned(self, other: Self) -> LazyDiffOwned {
        Self::DiffOwned {
            x: ::daft::DiffableOwned::diff_owned(self.x, other.x),
            y: ::daft::DiffableOwned::diff_owned(self.y, other.y),
        }
    }
}
