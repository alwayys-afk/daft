struct WithAttrsOwnedDiffOwned {
    a: Option<<i32 as ::daft::DiffableOwned>::DiffOwned>,
    b: Option<<BTreeMap<Uuid, BTreeSet<usize>> as ::daft::DiffableOwned>::DiffOwned>,
    d: Option<::daft::Leaf<Lazy>>,
    e: Option<::daft::Leaf<usize>>,
    f: Option<<usize as ::daft::DiffableOwned>::DiffOwned>,
}
impl ::core::fmt::Debug for WithAttrsOwnedDiffOwned
where
    Option<<i32 as ::daft::DiffableOwned>::DiffOwned>: ::core::fmt::Debug,
    Option<
        <BTreeMap<Uuid, BTreeSet<usize>> as ::daft::DiffableOwned>::DiffOwned,
    >: ::core::fmt::Debug,
    Option<::daft::Leaf<Lazy>>: ::core::fmt::Debug,
    Option<::daft::Leaf<usize>>: ::core::fmt::Debug,
    Option<<usize as ::daft::DiffableOwned>::DiffOwned>: ::core::fmt::Debug,
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
    Option<<i32 as ::daft::DiffableOwned>::DiffOwned>: ::core::cmp::PartialEq,
    Option<
        <BTreeMap<Uuid, BTreeSet<usize>> as ::daft::DiffableOwned>::DiffOwned,
    >: ::core::cmp::PartialEq,
    Option<::daft::Leaf<Lazy>>: ::core::cmp::PartialEq,
    Option<::daft::Leaf<usize>>: ::core::cmp::PartialEq,
    Option<<usize as ::daft::DiffableOwned>::DiffOwned>: ::core::cmp::PartialEq,
{
    fn eq(&self, other: &Self) -> bool {
        self.a == other.a && self.b == other.b && self.d == other.d && self.e == other.e
            && self.f == other.f
    }
}
impl ::core::cmp::Eq for WithAttrsOwnedDiffOwned
where
    Option<<i32 as ::daft::DiffableOwned>::DiffOwned>: ::core::cmp::Eq,
    Option<
        <BTreeMap<Uuid, BTreeSet<usize>> as ::daft::DiffableOwned>::DiffOwned,
    >: ::core::cmp::Eq,
    Option<::daft::Leaf<Lazy>>: ::core::cmp::Eq,
    Option<::daft::Leaf<usize>>: ::core::cmp::Eq,
    Option<<usize as ::daft::DiffableOwned>::DiffOwned>: ::core::cmp::Eq,
{}
impl ::daft::DiffableOwned for WithAttrsOwned {
    type DiffOwned = WithAttrsOwnedDiffOwned;
    fn diff_owned(self, other: Self) -> WithAttrsOwnedDiffOwned {
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
            d: if self.d == other.d {
                None
            } else {
                Some(::daft::Leaf {
                    before: self.d,
                    after: other.d,
                })
            },
            e: if self.e == other.e {
                None
            } else {
                Some(::daft::Leaf {
                    before: self.e,
                    after: other.e,
                })
            },
            f: if self.f == other.f {
                None
            } else {
                Some(::daft::DiffableOwned::diff_owned(self.f, other.f))
            },
        }
    }
}
struct LazyDiffOwned {
    x: Option<<usize as ::daft::DiffableOwned>::DiffOwned>,
    y: Option<<usize as ::daft::DiffableOwned>::DiffOwned>,
}
impl ::core::fmt::Debug for LazyDiffOwned
where
    Option<<usize as ::daft::DiffableOwned>::DiffOwned>: ::core::fmt::Debug,
    Option<<usize as ::daft::DiffableOwned>::DiffOwned>: ::core::fmt::Debug,
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
    Option<<usize as ::daft::DiffableOwned>::DiffOwned>: ::core::cmp::PartialEq,
    Option<<usize as ::daft::DiffableOwned>::DiffOwned>: ::core::cmp::PartialEq,
{
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y
    }
}
impl ::core::cmp::Eq for LazyDiffOwned
where
    Option<<usize as ::daft::DiffableOwned>::DiffOwned>: ::core::cmp::Eq,
    Option<<usize as ::daft::DiffableOwned>::DiffOwned>: ::core::cmp::Eq,
{}
impl ::daft::DiffableOwned for Lazy {
    type DiffOwned = LazyDiffOwned;
    fn diff_owned(self, other: Self) -> LazyDiffOwned {
        Self::DiffOwned {
            x: if self.x == other.x {
                None
            } else {
                Some(::daft::DiffableOwned::diff_owned(self.x, other.x))
            },
            y: if self.y == other.y {
                None
            } else {
                Some(::daft::DiffableOwned::diff_owned(self.y, other.y))
            },
        }
    }
}
