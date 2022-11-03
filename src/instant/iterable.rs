/// Special trait, generalizing all the types used to distinguish different types of iterable data sources.
/// Those types can be either type, implementing IntoIterator and Copy (any reference on IntoIterator suits)
/// or type, implementing Iterator and Clone.
/// There can be some problems when type implements Iterator and Copy simultaneously: since every Iterator
/// automatically implements IntoIterator, and Copy implementation requires Clone implementation too, such
/// type will suit both alternatives and will cause conflict until explicit Kind specified.
/// Trait is not sealed, so any user can define own kind and use it along with own Iterable implementation for that kind.
pub trait Kind {}

/// Type implementing Kind and used to mark types which are treated as IntoIterator and _not_ Iterator.
pub struct Source;

impl Kind for Source {}

/// Type implementing Kind and used to mark types which are treated as Iterator and _not_ IntoIterator.
pub struct Passage;

impl Kind for Passage {}

/// Trait used to generalize over IntoIterator+Copy and Iterator+Clone types.
/// Actual used source is defined via type parameter which should implement Kind trait.
/// There can be some problems when type implements Iterator and Copy simultaneously: since every Iterator
/// automatically implements IntoIterator, and Copy implementation requires Clone implementation too, such
/// type will suit both alternatives and will cause conflict until explicit Kind specified.
/// Trait is not sealed, so any user can define own Iterable implementation.
pub trait Iterable<K: Kind> {
    type Iter: Iterator;

    fn iter(&self) -> Self::Iter;
}

/// Implementation of Iterable for IntoIterator+Copy types - mostly for references onto IntoIterator types.
impl<T: IntoIterator + Copy> Iterable<Source> for T {
    type Iter = <Self as IntoIterator>::IntoIter;

    fn iter(&self) -> Self::Iter {
        IntoIterator::into_iter(*self)
    }
}

/// Implementation of Iterable for Iterator+Clone types.
impl<T: Iterator + Clone> Iterable<Passage> for T {
    type Iter = Self;

    fn iter(&self) -> Self::Iter {
        self.clone()
    }
}
