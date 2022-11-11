use core::{
    fmt::{Debug, Display, Formatter, Result as FmtResult},
    format_args,
};

/// Lets to output key-value pair regarding the propagated value of output alternativeness.
#[cfg_attr(docsrs, doc(cfg(feature = "field")))]
pub struct Field<'a, K: ?Sized, V: ?Sized> {
    key: &'a K,
    val: &'a V,
}

impl<'a, K: ?Sized, V: ?Sized> Field<'a, K, V> {
    /// Creates one [Field] examplar ready to be outputted.
    pub fn new(key: &'a K, val: &'a V) -> Self {
        Self { key, val }
    }
}

impl<'a, K: Display + ?Sized, V: Display + ?Sized> Display for Field<'a, K, V> {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        match f.alternate() {
            true => f.write_fmt(format_args!("{}: {:#}", self.key, self.val)),
            false => f.write_fmt(format_args!("{}: {}", self.key, self.val)),
        }
    }
}

impl<'a, K: Debug + ?Sized, V: Debug + ?Sized> Debug for Field<'a, K, V> {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        match f.alternate() {
            true => f.write_fmt(format_args!("{:?}: {:#?}", self.key, self.val)),
            false => f.write_fmt(format_args!("{:?}: {:?}", self.key, self.val)),
        }
    }
}
