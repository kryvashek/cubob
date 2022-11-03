use super::{Iterable, Kind};
use crate::{Alternate, ListShow};
use core::{
    fmt::{Display, Formatter, Result as FmtResult},
    marker::PhantomData,
};

/// Struct used to simplify displaying of any iterable lists.
pub struct InstantList<I, K> {
    alt: Alternate,
    val: I,
    _kind: PhantomData<K>,
}

impl<I, K> InstantList<I, K>
where
    K: Kind,
    I: Iterable<K>,
{
    /// Creates InstantList examplar with specified Alternate mode.
    pub fn new(alt: Alternate, val: I) -> Self {
        Self {
            alt,
            val,
            _kind: PhantomData,
        }
    }

    /// Creates InstantList examplar with Alternate::Inherit mode.
    pub fn inherit(val: I) -> Self {
        Self {
            alt: Alternate::Inherit,
            val,
            _kind: PhantomData,
        }
    }
}

impl<I, K> Display for InstantList<I, K>
where
    K: Kind,
    I: Iterable<K>,
    <I::Iter as Iterator>::Item: Display,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        ListShow::new(f, self.alt)
            .items_from_iter(self.val.iter())
            .finish()
    }
}
