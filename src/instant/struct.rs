use super::{Iterable, Kind};
use crate::{Alternate, DisplayPair, StructShow};
use core::{
    fmt::{Display, Formatter, Result as FmtResult},
    marker::PhantomData,
};

/// Struct used to simplify displaying of any iterable maps.
#[cfg_attr(docsrs, doc(cfg(all(feature = "struct", feature = "instant"))))]
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
    <I::Iter as Iterator>::Item: DisplayPair,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        StructShow::new(f, self.alt)
            .fields_from_iter(self.val.iter())
            .finish()
    }
}

#[cfg(feature = "embed")]
#[cfg_attr(docsrs, doc(cfg(all(feature = "embed", feature = "struct", feature = "instant"))))]
impl<I, K> crate::EmbedStruct for InstantStruct<I, K>
where
    K: Kind,
    I: Iterable<K>,
    <I::Iter as Iterator>::Item: DisplayPair,
{
    fn embed(&self, show: &mut StructShow) {
        show.fields_from_iter(self.val.iter());
    }
}
