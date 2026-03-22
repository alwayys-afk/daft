use crate::{Diffable, DiffableOwned, Leaf};
use newtype_uuid::{TypedUuid, TypedUuidKind};

impl<T> Diffable for TypedUuid<T>
where
    T: TypedUuidKind + Diffable,
{
    type Diff<'daft> = Leaf<&'daft TypedUuid<T>>;

    fn diff<'daft>(&'daft self, other: &'daft Self) -> Self::Diff<'daft> {
        Leaf { before: self, after: other }
    }
}

impl<T> DiffableOwned for TypedUuid<T>
where
    T: TypedUuidKind,
{
    type DiffOwned = Leaf<TypedUuid<T>>;

    fn diff_owned(self, other: Self) -> Self::DiffOwned {
        Leaf { before: self, after: other }
    }
}
