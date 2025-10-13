use core::fmt;

/// Helper type to mark something that shouldn't be outputted.
/// Used as replacer in some instantiations of [`Path`].
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct NoOutput;

impl fmt::Display for NoOutput {
    fn fmt(&self, _f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Ok(())
    }
}

/// Special type which helps to output different path-like types.
/// Everything that produces [`fmt::Display`] items while being iterated can be outputted this way.
/// Implements [`crate::Params`] and intended to be used as argument of [`crate::WithParams::with_params`].
/// ```
/// use cubob::{PathLike, WithParams};
/// 
/// let path_like_1 = vec!["A", "a", "1"];
/// let path_like_2 = [1, 2, 3, 4];
/// 
/// assert_eq!(path_like_1.with_params(PathLike::STRUCT).to_string(), "A.a.1");
/// assert_eq!(path_like_1.with_params(PathLike::LIST).to_string(), "A, a, 1");
/// assert_eq!(path_like_2.with_params(PathLike::FS_RELATIVE).to_string(), "1/2/3/4");
/// assert_eq!(path_like_2.with_params(PathLike::ROUTE).to_string(), "1->2->3->4");
/// 
/// ```
pub struct PathLike<D, R> {
    delimiter: D,
    replacer: R,
    prepend: bool,
}

impl PathLike<char, NoOutput> {
    pub const STRUCT: &'static Self = &Self {
        delimiter: '.',
        replacer: NoOutput,
        prepend: false,
    };
}

impl PathLike<char, char> {
    pub const FS_RELATIVE: &'static Self = &Self {
        delimiter: '/',
        replacer: '.',
        prepend: false,
    };
    pub const FS_ABSOLUTE: &'static Self = &Self {
        delimiter: '/',
        replacer: '.',
        prepend: true,
    };
}

impl PathLike<&'static str, NoOutput> {
    pub const ROUTE: &'static Self = &Self {
        delimiter: "->",
        replacer: NoOutput,
        prepend: false,
    };
    pub const LIST: &'static Self = &Self {
        delimiter: ", ",
        replacer: NoOutput,
        prepend: false
    };
}

impl<D> PathLike<D, NoOutput> {
    pub fn new(delimiter: D, prepend: bool) -> Self {
        Self { delimiter, replacer: NoOutput, prepend }
    }
}

impl<D1, R> PathLike<D1, R> {
    pub fn with_delimiter<D2>(self, delimiter: D2) -> PathLike<D2, R> {
        let Self {
            delimiter: _,
            replacer,
            prepend,
        } = self;
        PathLike {
            delimiter,
            replacer,
            prepend,
        }
    }
}

impl<D, R1> PathLike<D, R1> {
    pub fn with_replacer<R2>(self, replacer: R2) -> PathLike<D, R2> {
        let Self {
            delimiter,
            replacer: _,
            prepend,
        } = self;
        PathLike {
            delimiter,
            replacer,
            prepend,
        }
    }
}

impl<D, R> PathLike<D, R> {
    pub fn with_prepend(self, prepend: bool) -> Self {
        Self { prepend, ..self }
    }
}

impl<D, R, I> crate::Params<I> for PathLike<D, R>
where
    D: fmt::Display,
    R: fmt::Display,
    I: ?Sized,
    for <'a> &'a I: IntoIterator<Item: fmt::Display>,
{
    fn fmt(&self, value: &I, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut iter = value.into_iter();

        let Some(first) = iter.next() else {
            return self.replacer.fmt(f);
        };

        match self.prepend {
            false => fmt::Display::fmt(&first, f),
            true => output_component(f, &self.delimiter, first),
        }?;

        while let Some(item) = iter.next() {
            output_component(f, &self.delimiter, item)?;
        }

        Ok(())
    }
}


fn output_component(
    f: &mut fmt::Formatter<'_>,
    delimiter: impl fmt::Display,
    item: impl fmt::Display,
) -> fmt::Result {
    write!(f, "{delimiter}{item}")
}
