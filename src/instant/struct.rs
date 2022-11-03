use super::{Iterable, Kind};
use crate::{Alternate, Pair, StructShow};
use core::{
    fmt::{Display, Formatter, Result as FmtResult},
    marker::PhantomData,
};

/// Struct used to simplify displaying of any iterable maps.
pub struct InstantStruct<I, K> {
    alt: Alternate,
    val: I,
    _kind: PhantomData<K>,
}

impl<I, K> InstantStruct<I, K>
where
    K: Kind,
    I: Iterable<K>,
{
    /// Creates InstantStruct examplar with specified Alternate mode.
    pub fn new(alt: Alternate, val: I) -> Self {
        Self {
            alt,
            val,
            _kind: PhantomData,
        }
    }

    /// Creates InstantStruct examplar with specified Alternate mode.
    pub fn inherit(val: I) -> Self {
        Self {
            alt: Alternate::Inherit,
            val,
            _kind: PhantomData,
        }
    }
}

impl<I, K> Display for InstantStruct<I, K>
where
    K: Kind,
    I: Iterable<K>,
    <I::Iter as Iterator>::Item: Pair,
    <<I::Iter as Iterator>::Item as Pair>::Left: Display,
    <<I::Iter as Iterator>::Item as Pair>::Right: Display,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        StructShow::new(f, self.alt)
            .fields_from_iter(self.val.iter())
            .finish()
    }
}
