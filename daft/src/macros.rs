//! Macros for internal implementations.

macro_rules! leaf_owned {
    ($($typ:ty),*) => {
        $(
            impl $crate::DiffableOwned for $typ {
                type DiffOwned = $crate::Leaf<Self>;

                fn diff_owned(self, other: Self) -> Self::DiffOwned {
                    $crate::Leaf {
                        before: self,
                        after: other
                    }
                }
            }
        )*
    }
}

macro_rules! leaf {
    ($($typ:ty),*) => {
        $(
            impl $crate::Diffable for $typ {
                type Diff<'daft> = $crate::Leaf<&'daft Self>;

                fn diff<'daft>(&'daft self, other: &'daft Self) -> Self::Diff<'daft> {
                    $crate::Leaf {
                        before: self,
                        after: other
                    }
                }
            }
        )*
    }
}

#[cfg(feature = "alloc")]
macro_rules! leaf_deref {
    ($($typ:ty => $target:ty),*) => {
        $(
            impl $crate::Diffable for $typ {
                type Diff<'daft> = $crate::Leaf<&'daft $target>;

                fn diff<'daft>(&'daft self, other: &'daft Self) -> Self::Diff<'daft> {
                    $crate::Leaf {
                        before: &**self,
                        after: &**other
                    }
                }
            }
        )*
    };
}

/// Create a type `<MapType>Diff` and `impl Diffable` on it.
///
/// This is supported for `BTreeMap` and `HashMap`
#[cfg(feature = "alloc")]
macro_rules! map_diff {
    ($(#[$doc:meta])* $typ:ident, $key_constraint:ident) => {
         paste::paste! {
            $(#[$doc])*
            #[derive(Debug, PartialEq, Eq)]
            pub struct [<$typ Diff>]<'daft, K: $key_constraint + Eq, V> {
                /// Entries common to both maps.
                ///
                /// Values are stored as `Leaf`s to references.
                pub common: $typ<&'daft K, $crate::Leaf<&'daft V>>,

                /// Entries present in the `after` map, but not in `before`.
                pub added: $typ<&'daft K, &'daft V>,

                /// Entries present in the `before` map, but not in `after`.
                pub removed: $typ<&'daft K, &'daft V>,
            }

            impl<'daft, K: $key_constraint + Eq, V> [<$typ Diff>]<'daft, K, V> {
                #[doc = "Create a new, empty `" $typ "Diff` instance."]
                pub fn new() -> Self {
                    Self {
                        common: $typ::new(),
                        added: $typ::new(),
                        removed: $typ::new(),
                    }
                }
            }

            impl<'daft, K: $key_constraint + Eq, V: Eq> [<$typ Diff>]<'daft, K, V> {
                /// Return an iterator over unchanged keys and values.
                pub fn unchanged(&self) -> impl Iterator<Item = (&'daft K, &'daft V)> + '_ {
                    self.common.iter().filter_map(|(k, leaf)| {
                        leaf.is_unchanged().then_some((*k, leaf.before))
                    })
                }

                /// Return true if the value corresponding to the key is
                /// unchanged.
                pub fn is_unchanged(&self, key: &K) -> bool {
                    self.common.get(key).is_some_and(|leaf| leaf.is_unchanged())
                }

                /// Return the value associated with the key if it is unchanged,
                /// otherwise `None`.
                pub fn get_unchanged(&self, key: &K) -> Option<&'daft V> {
                    self.common.get(key).and_then(|leaf| {
                        leaf.is_unchanged().then_some(leaf.before)
                    })
                }

                /// Return an iterator over unchanged keys.
                pub fn unchanged_keys(&self) -> impl Iterator<Item = &'daft K> + '_ {
                    self.common.iter().filter_map(|(k, leaf)| {
                        leaf.is_unchanged().then_some(*k)
                    })
                }

                /// Return an iterator over unchanged values.
                pub fn unchanged_values(&self) -> impl Iterator<Item = &'daft V> + '_ {
                    self.common.iter().filter_map(|(_, leaf)| {
                        leaf.is_unchanged().then_some(leaf.before)
                    })
                }

                /// Return an iterator over modified keys and values.
                pub fn modified(&self) -> impl Iterator<Item = (&'daft K, $crate::Leaf<&'daft V>)> + '_ {
                    self.common.iter().filter_map(|(k, leaf)| {
                        leaf.is_modified().then_some((*k, *leaf))
                    })
                }

                /// Return true if the value corresponding to the key is
                /// modified.
                pub fn is_modified(&self, key: &K) -> bool {
                    self.common.get(key).is_some_and(|leaf| leaf.is_modified())
                }

                /// Return the `Leaf` associated with the key if it is modified,
                /// otherwise `None`.
                pub fn get_modified(&self, key: &K) -> Option<$crate::Leaf<&'daft V>> {
                    self.common.get(key).and_then(|leaf| {
                        leaf.is_modified().then_some(*leaf)
                    })
                }

                /// Return an iterator over modified keys.
                pub fn modified_keys(&self) -> impl Iterator<Item = &'daft K> + '_ {
                    self.common.iter().filter_map(|(k, leaf)| {
                        leaf.is_modified().then_some(*k)
                    })
                }

                /// Return an iterator over modified values.
                pub fn modified_values(&self) -> impl Iterator<Item = $crate::Leaf<&'daft V>> + '_ {
                    self.common.iter().filter_map(|(_, leaf)| {
                        leaf.is_modified().then_some(*leaf)
                    })
                }

                /// Return an iterator over modified keys and values, performing
                /// a diff on the values.
                ///
                /// This is useful when `V::Diff` is a complex type, not just a
                /// [`Leaf`](crate::Leaf).
                #[allow(rustdoc::redundant_explicit_links)] // some macro use sites have Leaf available, some don't
                pub fn modified_diff(&self) -> impl Iterator<Item = (&'daft K, V::Diff<'daft>)> + '_
                where
                    V: Diffable,
                {
                    self.modified().map(|(k, leaf)| (k, leaf.before.diff(&leaf.after)))
                }

                /// Return an iterator over modified values, performing a diff on
                /// them.
                ///
                /// This is useful when `V::Diff` is a complex type, not just a
                /// [`Leaf`](crate::Leaf).
                #[allow(rustdoc::redundant_explicit_links)] // some macro use sites have Leaf available, some don't
                pub fn modified_values_diff(&self) -> impl Iterator<Item = V::Diff<'daft>> + '_
                where
                    V: Diffable,
                {
                    self.modified_values().map(|leaf| leaf.before.diff(&leaf.after))
                }
            }

            // Note: not deriving Default here because we don't want to require
            // K or V to be Default.
            impl<'daft, K: $key_constraint + Eq, V> Default for [<$typ Diff>]<'daft, K, V> {
                fn default() -> Self {
                    Self::new()
                }
            }

            impl<K: $key_constraint + Eq, V> $crate::Diffable for $typ<K, V>
            {
                type Diff<'daft> = [<$typ Diff>]<'daft, K, V> where K: 'daft, V: 'daft;

                fn diff<'daft>(&'daft self, other: &'daft Self) -> Self::Diff<'daft> {
                    let mut diff = [<$typ Diff>]::new();
                    for (k, v) in self {
                        if let Some(other_v) = other.get(k) {
                            diff.common.insert(k, $crate::Leaf { before: v, after: other_v });
                        } else {
                            diff.removed.insert(k, v);
                        }
                    }
                    for (k, v) in other {
                        if !self.contains_key(k) {
                            diff.added.insert(k, v);
                        }
                    }
                    diff
                }
            }
        }
    }
}

/// Create a type `<SetType>Diff` and `impl Diffable` on it.
///
/// This is supported for `BTreeSet` and `HashSet`.
#[cfg(feature = "alloc")]
macro_rules! set_diff {
    ($(#[$doc:meta])* $typ:ident, $key_constraint:ident) => {
        paste::paste! {
            $(#[$doc])*
            #[derive(Debug, PartialEq, Eq)]
            pub struct [<$typ Diff>]<'daft, K: $key_constraint + Eq> {
                /// Entries common to both sets.
                pub common: $typ<&'daft K>,

                /// Entries present in the `after` set, but not in `before`.
                pub added: $typ<&'daft K>,

                /// Entries present in the `before` set, but not in `after`.
                pub removed: $typ<&'daft K>,
            }

            impl<'daft, K: $key_constraint + Eq> [<$typ Diff>]<'daft, K> {
                #[doc = "Create a new, empty `" $typ "Diff` instance."]
                pub fn new() -> Self {
                    Self {
                        common: $typ::new(),
                        added: $typ::new(),
                        removed: $typ::new(),
                    }
                }
            }

            // Note: not deriving Default here because we don't want to require
            // K to be Default.
            impl<'daft, K: $key_constraint + Eq> Default for [<$typ Diff>]<'daft, K> {
                fn default() -> Self {
                    Self::new()
                }
            }

            impl<K: $key_constraint + Eq>
                $crate::Diffable for $typ<K>
            {
                type Diff<'daft> = [<$typ Diff>]<'daft, K> where K: 'daft;

                fn diff<'daft>(&'daft self, other: &'daft Self) -> Self::Diff<'daft> {
                    let mut diff = [<$typ Diff>]::new();
                    diff.removed = self.difference(other).collect();
                    diff.added = other.difference(self).collect();
                    diff.common = self.intersection(other).collect();
                    diff
                }
            }
        }
    }
}

/// Create a type `<MapType>DiffOwned` and `impl DiffableOwned` on it.
///
/// This is supported for `BTreeMap` and `HashMap`.
#[cfg(feature = "alloc")]
macro_rules! map_diff_owned {
    ($(#[$doc:meta])* $typ:ident, $key_constraint:ident) => {
        paste::paste! {
            $(#[$doc])*
            #[derive(Clone, Debug, PartialEq, Eq)]
            #[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
            pub struct [<$typ DiffOwned>]<K: $key_constraint + Eq, V> {
                /// Entries common to both maps.
                ///
                /// Values are stored as owned `Leaf`s.
                pub common: $typ<K, $crate::Leaf<V>>,

                /// Entries present in the `after` map, but not in `before`.
                pub added: $typ<K, V>,

                /// Entries present in the `before` map, but not in `after`.
                pub removed: $typ<K, V>,
            }

            impl<K: $key_constraint + Eq, V> [<$typ DiffOwned>]<K, V> {
                #[doc = "Create a new, empty `" $typ "DiffOwned` instance."]
                pub fn new() -> Self {
                    Self {
                        common: $typ::new(),
                        added: $typ::new(),
                        removed: $typ::new(),
                    }
                }
            }

            impl<K: $key_constraint + Eq, V: Eq> [<$typ DiffOwned>]<K, V> {
                /// Return an iterator over unchanged keys and values.
                pub fn unchanged(&self) -> impl Iterator<Item = (&K, &V)> + '_ {
                    self.common.iter().filter_map(|(k, leaf)| {
                        leaf.is_unchanged().then_some((k, &leaf.before))
                    })
                }

                /// Return true if the value corresponding to the key is
                /// unchanged.
                pub fn is_unchanged(&self, key: &K) -> bool {
                    self.common.get(key).is_some_and(|leaf| leaf.is_unchanged())
                }

                /// Return the value associated with the key if it is unchanged,
                /// otherwise `None`.
                pub fn get_unchanged(&self, key: &K) -> Option<&V> {
                    self.common.get(key).and_then(|leaf| {
                        leaf.is_unchanged().then_some(&leaf.before)
                    })
                }

                /// Return an iterator over unchanged keys.
                pub fn unchanged_keys(&self) -> impl Iterator<Item = &K> + '_ {
                    self.common.iter().filter_map(|(k, leaf)| {
                        leaf.is_unchanged().then_some(k)
                    })
                }

                /// Return an iterator over unchanged values.
                pub fn unchanged_values(&self) -> impl Iterator<Item = &V> + '_ {
                    self.common.iter().filter_map(|(_, leaf)| {
                        leaf.is_unchanged().then_some(&leaf.before)
                    })
                }

                /// Return an iterator over modified keys and values.
                pub fn modified(&self) -> impl Iterator<Item = (&K, $crate::Leaf<&V>)> + '_ {
                    self.common.iter().filter_map(|(k, leaf)| {
                        leaf.is_modified().then_some((k, leaf.as_ref()))
                    })
                }

                /// Return true if the value corresponding to the key is
                /// modified.
                pub fn is_modified(&self, key: &K) -> bool {
                    self.common.get(key).is_some_and(|leaf| leaf.is_modified())
                }

                /// Return the `Leaf` associated with the key if it is modified,
                /// otherwise `None`.
                pub fn get_modified(&self, key: &K) -> Option<$crate::Leaf<&V>> {
                    self.common.get(key).and_then(|leaf| {
                        leaf.is_modified().then_some(leaf.as_ref())
                    })
                }

                /// Return an iterator over modified keys.
                pub fn modified_keys(&self) -> impl Iterator<Item = &K> + '_ {
                    self.common.iter().filter_map(|(k, leaf)| {
                        leaf.is_modified().then_some(k)
                    })
                }

                /// Return an iterator over modified values.
                pub fn modified_values(&self) -> impl Iterator<Item = $crate::Leaf<&V>> + '_ {
                    self.common.iter().filter_map(|(_, leaf)| {
                        leaf.is_modified().then_some(leaf.as_ref())
                    })
                }
            }

            // Note: not deriving Default here because we don't want to require
            // K or V to be Default.
            impl<K: $key_constraint + Eq, V> Default for [<$typ DiffOwned>]<K, V> {
                fn default() -> Self {
                    Self::new()
                }
            }

            impl<K: $key_constraint + Eq, V> $crate::DiffableOwned for $typ<K, V>
            {
                type DiffOwned = [<$typ DiffOwned>]<K, V>;

                fn diff_owned(self, mut other: Self) -> Self::DiffOwned {
                    let mut diff = [<$typ DiffOwned>]::new();
                    for (k, v) in self {
                        if let Some(other_v) = other.remove(&k) {
                            diff.common.insert(k, $crate::Leaf { before: v, after: other_v });
                        } else {
                            diff.removed.insert(k, v);
                        }
                    }
                    for (k, v) in other {
                        diff.added.insert(k, v);
                    }
                    diff
                }
            }
        }
    }
}

/// Create a type `<SetType>DiffOwned` and `impl DiffableOwned` on it.
///
/// This is supported for `BTreeSet` and `HashSet`.
#[cfg(feature = "alloc")]
macro_rules! set_diff_owned {
    ($(#[$doc:meta])* $typ:ident, $key_constraint:ident) => {
        paste::paste! {
            $(#[$doc])*
            #[derive(Clone, Debug, PartialEq, Eq)]
            #[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
            pub struct [<$typ DiffOwned>]<K: $key_constraint + Eq> {
                /// Entries common to both sets.
                pub common: $typ<K>,

                /// Entries present in the `after` set, but not in `before`.
                pub added: $typ<K>,

                /// Entries present in the `before` set, but not in `after`.
                pub removed: $typ<K>,
            }

            impl<K: $key_constraint + Eq> [<$typ DiffOwned>]<K> {
                #[doc = "Create a new, empty `" $typ "DiffOwned` instance."]
                pub fn new() -> Self {
                    Self {
                        common: $typ::new(),
                        added: $typ::new(),
                        removed: $typ::new(),
                    }
                }
            }

            // Note: not deriving Default here because we don't want to require
            // K to be Default.
            impl<K: $key_constraint + Eq> Default for [<$typ DiffOwned>]<K> {
                fn default() -> Self {
                    Self::new()
                }
            }

            impl<K: $key_constraint + Eq> $crate::DiffableOwned for $typ<K>
            {
                type DiffOwned = [<$typ DiffOwned>]<K>;

                fn diff_owned(self, mut other: Self) -> Self::DiffOwned {
                    let mut diff = [<$typ DiffOwned>]::new();
                    for k in self {
                        if other.remove(&k) {
                            diff.common.insert(k);
                        } else {
                            diff.removed.insert(k);
                        }
                    }
                    diff.added = other;
                    diff
                }
            }
        }
    }
}
