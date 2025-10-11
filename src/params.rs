use core::fmt;

/// Intended to perform output of given type T examplar according to inner state.
/// The same type may implement Params to output different types.
/// Designed to be used either with [`Parameterized`] (see examples below) or with [`crate::Custom`] (see current example).
#[cfg_attr(
    feature = "custom",
    doc = r##"
```
use core::fmt;
use cubob::Params;

struct PairOutput {
    delimiter: &'static str
}

impl PairOutput {
    fn output(&self, a: impl fmt::Display, b: impl fmt::Display, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}{}{}", a, self.delimiter, b)
    }
}

impl<A, B> Params<(A, B)> for PairOutput
where
    A: fmt::Display,
    B: fmt::Display
{
    fn fmt(&self, value: &(A, B), f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.output(&value.0, &value.1, f)
    }
}

impl<T> Params<[T; 2]> for PairOutput
where
    T: fmt::Display
{
    fn fmt(&self, value: &[T; 2], f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.output(&value[0], &value[1], f)
    }
}

let pair_1 = (1, "two");
let pair_2 = [3.4, 5.6];

let pair_output_1 = PairOutput { delimiter: ": " };
let pair_output_2 = PairOutput { delimiter: "@" };

let custom_1 = cubob::Custom::new(&pair_1, |pair, f: &mut fmt::Formatter<'_>| pair_output_1.fmt(pair, f));
let custom_2 = cubob::Custom::new(&pair_2, |pair, f: &mut fmt::Formatter<'_>| pair_output_1.fmt(pair, f));
let custom_3 = cubob::Custom::new(&pair_1, |pair, f: &mut fmt::Formatter<'_>| pair_output_2.fmt(pair, f));
let custom_4 = cubob::Custom::new(&pair_2, |pair, f: &mut fmt::Formatter<'_>| pair_output_2.fmt(pair, f));

assert_eq!(custom_1.to_string(), "1: two");
assert_eq!(custom_2.to_string(), "3.4: 5.6");
assert_eq!(custom_3.to_string(), "1@two");
assert_eq!(custom_4.to_string(), "3.4@5.6");
```
"##
)]
#[cfg_attr(docsrs, doc(cfg(feature = "params")))]
pub trait Params<T: ?Sized> {
    fn fmt(&self, value: &T, f: &mut fmt::Formatter<'_>) -> fmt::Result;
}

/// Implements actual [`fmt::Display`] output for referenced examplar of type T according to given Params.
/// Compare with the example above.
/// ```
/// use core::fmt;
/// use cubob::{Params, Parameterized};
///
/// struct PairOutput {
///     delimiter: &'static str
/// }
///
/// impl PairOutput {
///     fn output(&self, a: impl fmt::Display, b: impl fmt::Display, f: &mut fmt::Formatter<'_>) -> fmt::Result {
///         write!(f, "{}{}{}", a, self.delimiter, b)
///     }
/// }
///
/// impl<A, B> Params<(A, B)> for PairOutput
/// where
///     A: fmt::Display,
///     B: fmt::Display
/// {
///     fn fmt(&self, value: &(A, B), f: &mut fmt::Formatter<'_>) -> fmt::Result {
///         self.output(&value.0, &value.1, f)
///     }
/// }
///
/// impl<T> Params<[T; 2]> for PairOutput
/// where
///     T: fmt::Display
/// {
///     fn fmt(&self, value: &[T; 2], f: &mut fmt::Formatter<'_>) -> fmt::Result {
///         self.output(&value[0], &value[1], f)
///     }
/// }
///
/// let pair_1 = (1, "two");
/// let pair_2 = [3.4, 5.6];
///
/// let pair_output_1 = PairOutput { delimiter: ": " };
/// let pair_output_2 = PairOutput { delimiter: "@" };
///
/// let parametrized_1 = Parameterized::new(&pair_1, &pair_output_1);
/// let parametrized_2 = Parameterized::new(&pair_2, &pair_output_1);
/// let parametrized_3 = Parameterized::new(&pair_1, &pair_output_2);
/// let parametrized_4 = Parameterized::new(&pair_2, &pair_output_2);
///
/// assert_eq!(parametrized_1.to_string(), "1: two");
/// assert_eq!(parametrized_2.to_string(), "3.4: 5.6");
/// assert_eq!(parametrized_3.to_string(), "1@two");
/// assert_eq!(parametrized_4.to_string(), "3.4@5.6");
/// ```
#[cfg_attr(docsrs, doc(cfg(feature = "params")))]
pub struct Parameterized<'a, T: ?Sized, P: ?Sized> {
    value: &'a T,
    params: &'a P,
}

impl<'a, T, P> Parameterized<'a, T, P> {
    /// Creates new instance, referencing related value and params.
    pub fn new(value: &'a T, params: &'a P) -> Self {
        Self { value, params }
    }
}

impl<'a, T, P> fmt::Display for Parameterized<'a, T, P>
where
    P: Params<T>,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.params.fmt(self.value, f)
    }
}

/// Simplifies outputting with some Params implementor for type which implements WithParams.
/// ```
/// use core::fmt;
/// use cubob::{Params, WithParams};
///
/// struct PairOutput {
///     delimiter: &'static str
/// }
///
/// impl PairOutput {
///     fn output(&self, a: impl fmt::Display, b: impl fmt::Display, f: &mut fmt::Formatter<'_>) -> fmt::Result {
///         write!(f, "{}{}{}", a, self.delimiter, b)
///     }
/// }
///
/// impl<A, B> Params<(A, B)> for PairOutput
/// where
///     A: fmt::Display,
///     B: fmt::Display
/// {
///     fn fmt(&self, value: &(A, B), f: &mut fmt::Formatter<'_>) -> fmt::Result {
///         self.output(&value.0, &value.1, f)
///     }
/// }
///
/// impl<T> Params<[T; 2]> for PairOutput
/// where
///     T: fmt::Display
/// {
///     fn fmt(&self, value: &[T; 2], f: &mut fmt::Formatter<'_>) -> fmt::Result {
///         self.output(&value[0], &value[1], f)
///     }
/// }
///
/// let pair_1 = (1, "two");
/// let pair_2 = [3.4, 5.6];
///
/// let pair_output_1 = PairOutput { delimiter: ": " };
/// let pair_output_2 = PairOutput { delimiter: "@" };
///
/// let custom_1 = (&pair_1).with_params(&pair_output_1);
/// let custom_2 = (&pair_2).with_params(&pair_output_1);
/// let custom_3 = (&pair_1).with_params(&pair_output_2);
/// let custom_4 = (&pair_2).with_params(&pair_output_2);
///
/// assert_eq!(custom_1.to_string(), "1: two");
/// assert_eq!(custom_2.to_string(), "3.4: 5.6");
/// assert_eq!(custom_3.to_string(), "1@two");
/// assert_eq!(custom_4.to_string(), "3.4@5.6");
/// ```
#[cfg_attr(docsrs, doc(cfg(feature = "params")))]
pub trait WithParams<P>: Sized {
    fn with_params<'a>(&'a self, params: &'a P) -> Parameterized<'a, Self, P>;
}

impl<T, P> WithParams<P> for T
where
    P: Params<T>,
{
    fn with_params<'a>(&'a self, params: &'a P) -> Parameterized<'a, Self, P> {
        Parameterized::new(self, params)
    }
}
