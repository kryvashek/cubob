use core::fmt;

/// Wraps some value and its output function to be used for output.
/// ```
/// use core::fmt;
/// use cubob::Custom;
///
/// let pair_output_func = |value: &(&str, &str), f: &mut fmt::Formatter<'_>| {
///     write!(f, "{}: {}", value.0, value.1)
/// };
/// let output_1 = ("field_1", "field_1_value");
/// let mut custom = Custom::new(&output_1, pair_output_func);
/// assert_eq!(format!("{custom}"), "field_1: field_1_value");
///
/// let output_2 = ("field_2", "field_2_value");
/// custom = custom.with_value(&output_2);
/// assert_eq!(format!("{custom}"), "field_2: field_2_value");
/// ```
#[cfg_attr(docsrs, doc(cfg(feature = "custom")))]
pub struct Custom<T, F> {
    value: T,
    func: F,
}

impl<T, F> Custom<T, F> {
    /// Creates new instance of custom outputter.
    /// ```
    /// let custom = cubob::Custom::new(&(1, 2), |v,f| { write!(f, "{}: {}", v.0, v.1) });
    /// assert_eq!(format!("{custom}"), "1: 2");
    /// ```
    pub fn new(value: T, func: F) -> Self
    where
        F: Fn(T, &mut fmt::Formatter<'_>) -> fmt::Result,
    {
        Self { value, func }
    }

    /// Replaces the value to output with another one.
    /// Creates new instance of custom outputter.
    /// ```
    /// let mut custom = cubob::Custom::new(&(1, 2), |v,f| { write!(f, "{}: {}", v.0, v.1) });
    /// assert_eq!(format!("{custom}"), "1: 2");
    /// custom = custom.with_value(&(3, 4));
    /// assert_eq!(format!("{custom}"), "3: 4");
    /// ```
    pub fn with_value(self, value: T) -> Self {
        Self { value, ..self }
    }
}

impl<T, F> fmt::Display for Custom<T, F>
where
    T: Clone,
    F: Fn(T, &mut fmt::Formatter<'_>) -> fmt::Result,
{
    /// Implements outputting of current Custom instance, which is defined by given value and function.
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        (self.func)(self.value.clone(), f)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn output_vector_as_path() {
        let output_func = |v: &Vec<_>, f: &mut fmt::Formatter<'_>| {
            let mut v_items = v.iter();
            let Some(first) = v_items.next() else {
                return Ok(());
            };
            write!(f, "{first}")?;
            for item in v_items {
                write!(f, ".{item}")?;
            }
            Ok(())
        };

        let v1 = vec![];
        let mut custom = Custom::new(&v1, output_func);
        let text = format!("{custom}");
        assert_eq!(text, "");

        let v2 = vec![1];
        custom = custom.with_value(&v2);
        let text = format!("{custom}");
        assert_eq!(text, "1");

        let v1 = vec![1, 2];
        custom = custom.with_value(&v1);
        let text = format!("{custom}");
        assert_eq!(text, "1.2");

        let v2 = vec![1, 2, 3];
        custom = custom.with_value(&v2);
        let text = format!("{custom}");
        assert_eq!(text, "1.2.3");
    }
}
