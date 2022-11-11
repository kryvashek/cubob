use core::fmt::Display;

/// Trait used to generalize over tuples of displayable types
/// and references onto such tuples.
/// Trait is not sealed and can be implementd for any other needed type.
#[cfg_attr(docsrs, doc(cfg(feature = "struct")))]
pub trait DisplayPair {
    /// Type of the left-side variable in corresponding 'left: right' construction.
    type Left: Display;
    /// Type of the right-side variable in corresponding 'left: right' construction.
    type Right: Display;

    /// Return a reference onto the left-side variable in corresponding 'left: right' construction.
    fn left(&self) -> &Self::Left;

    /// Return a reference onto the right-side variable in corresponding 'left: right' construction.
    fn rifgt(&self) -> &Self::Right;
}

impl<L: Display, R: Display> DisplayPair for (L, R) {
    type Left = L;
    type Right = R;

    fn left(&self) -> &Self::Left {
        &self.0
    }

    fn rifgt(&self) -> &Self::Right {
        &self.1
    }
}

impl<L: Display, R: Display> DisplayPair for &(L, R) {
    type Left = L;
    type Right = R;

    fn left(&self) -> &Self::Left {
        &self.0
    }

    fn rifgt(&self) -> &Self::Right {
        &self.1
    }
}
