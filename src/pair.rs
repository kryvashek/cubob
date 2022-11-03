use core::fmt::Display;

/// Trait used to generalize over tuples of displayable types
/// and references onto such tuples.
/// Trait is not sealed and can be implementd for any other needed type.
pub trait Pair {
    type Left: Display;
    type Right: Display;

    fn left(&self) -> &Self::Left;
    fn rifgt(&self) -> &Self::Right;
}

impl<L: Display, R: Display> Pair for (L, R) {
    type Left = L;
    type Right = R;

    fn left(&self) -> &Self::Left {
        &self.0
    }

    fn rifgt(&self) -> &Self::Right {
        &self.1
    }
}

impl<L: Display, R: Display> Pair for &(L, R) {
    type Left = L;
    type Right = R;

    fn left(&self) -> &Self::Left {
        &self.0
    }

    fn rifgt(&self) -> &Self::Right {
        &self.1
    }
}
