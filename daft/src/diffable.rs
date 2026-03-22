/// Represents a type which can be diffed.
///
/// For more information, see the [crate-level documentation](crate).
pub trait Diffable {
    /// The type of the diff.
    ///
    /// This is a [generic associated type][GAT], also known as a GAT. The
    /// `'daft` lifetime is used in the `diff` method to ensure that the
    /// returned diff is valid for the lifetime of the input values.
    ///
    /// [GAT]: https://blog.rust-lang.org/2021/08/03/GATs-stabilization-push.html
    type Diff<'daft>
    where
        Self: 'daft;

    /// Compute the diff between two values.
    fn diff<'daft>(&'daft self, other: &'daft Self) -> Self::Diff<'daft>;
}

/// Represents a type which can be diffed by consuming both values.
///
/// Unlike [`Diffable`], which borrows values and returns a diff tied to
/// their lifetime, this trait takes ownership of both `self` and `other`,
/// producing a diff with no lifetime parameter. This is useful when the
/// diff needs to be serialized, stored, or otherwise outlive the original
/// values.
pub trait DiffableOwned: Sized {
    /// The type of the owned diff. Has no lifetime parameter.
    type DiffOwned;

    /// Compute the diff between two values, consuming both.
    fn diff_owned(self, other: Self) -> Self::DiffOwned;
}
