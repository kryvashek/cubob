//! Some primitives to simplify implementation of structured data output in display mode.
//! Usage example:
//! ```
//! use core::fmt::{Display, Formatter, Result as FmtResult};
//! use cubob::display_struct;
//!
//! struct Point {
//!     x: i32,
//!     y: i32,
//! }
//!
//! struct Line {
//!     a: Point,
//!     b: Point,
//! }
//!
//! impl Display for Point {
//!     fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
//!         display_struct(
//!             f,
//!             &[
//!                 (&"x", &self.x),
//!                 (&"y", &self.y),
//!             ],
//!         )
//!     }
//! }
//!
//! impl Display for Line {
//!     fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
//!         display_struct(
//!             f,
//!             &[
//!                 (&"a", &self.a),
//!                 (&"b", &self.b),
//!             ],
//!         )
//!     }
//! }
//!
//! fn main() {
//!     let line = Line{ a: Point{ x: 0, y: 0}, b: Point{ x: 1, y: 1} };
//!     println!("One-line: {}", line);
//!     println!("Prettified: {:#}", line);
//! }
//! ```

use core::{
    fmt::{Debug, DebugList, DebugSet, Display, Formatter, Result as FmtResult},
    format_args,
};

/// Lets to output key-value pair regarding the propagated value of output alternativeness.
pub struct Field<'a, K: ?Sized, V: ?Sized> {
    key: &'a K,
    val: &'a V,
}

impl<'a, K: ?Sized, V: ?Sized> Field<'a, K, V> {
    /// Creates one Field examplar ready to be outputted.
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

/// Alternate mode to use while outputting.
pub enum Alternate {
    /// Output data in one line (matches Formatter::alternate() == false).
    OneLine,
    /// Output data in prettified format (matches Formatter::alternate() == true).
    Pretty,
    /// Output data in format regarding alternate mode of given Formatter examplar.
    Inherit,
}

type StructEntrier<'t> = &'t dyn Fn(&mut DebugSet<'_, '_>, &dyn Display, &dyn Display);

/// Lets to output some structure regarding the propagated value of output alternativeness.
pub struct StructShow<'a, 'b> {
    wrapper: DebugSet<'a, 'b>,
    entrier: StructEntrier<'static>,
}

impl<'a, 'b> StructShow<'a, 'b> {
    const USUAL_ENTRIER: StructEntrier<'static> = &|w, k, v| {
        w.entry(&format_args!("{}: {}", k, v));
    };
    const ALT_ENTRIER: StructEntrier<'static> = &|w, k, v| {
        w.entry(&format_args!("{}: {:#}", k, v));
    };
    const NULL_ENTRIER: StructEntrier<'static> = &|_w, _k, _v| {};

    /// Creates one StructShow examplar starting its output.
    pub fn new(formatter: &'a mut Formatter<'b>, alternate: Alternate) -> Self {
        let entrier = match alternate {
            Alternate::OneLine => Self::USUAL_ENTRIER,
            Alternate::Pretty => Self::ALT_ENTRIER,
            Alternate::Inherit => match formatter.alternate() {
                true => Self::ALT_ENTRIER,
                false => Self::USUAL_ENTRIER,
            },
        };
        Self {
            wrapper: formatter.debug_set(),
            entrier,
        }
    }

    /// Adds one key-value pair to the struct output.
    pub fn field(&mut self, key: &dyn Display, val: &dyn Display) -> &mut Self {
        (self.entrier)(&mut self.wrapper, key, val);
        self
    }

    /// Adds one optional key-value pair to the struct output if its value matches Some(_).
    pub fn field_opt<T: Display>(&mut self, key: &dyn Display, val: &Option<T>) -> &mut Self {
        if let Some(actual_value) = val {
            self.field(key, actual_value);
        }
        self
    }

    /// Finishes the struct output, returning the result.
    pub fn finish(&mut self) -> FmtResult {
        self.entrier = Self::NULL_ENTRIER;
        self.wrapper.finish()
    }

    /// Adds several key-value pair to the struct output from slice.
    pub fn fields(&mut self, fields: &[(&dyn Display, &dyn Display)]) -> &mut Self {
        self.fields_from_iter(fields.iter().map(|(k, v)| (k, v)))
    }

    /// Adds several key-value pair to the struct output from iterator.
    pub fn fields_from_iter<'c, K, V, I>(&mut self, fields: I) -> &mut Self
    where
        K: Display + 'c,
        V: Display + 'c,
        I: Iterator<Item = (K, V)> + 'c,
    {
        fields.for_each(|(key, val)| (self.entrier)(&mut self.wrapper, &key, &val));
        self
    }
}

/// Performs the whole struct output routine from creation of StructShow examplar to finishing (for example see the module-level documentation).
/// Works with slice, always inherits alternate mode.
pub fn display_struct(f: &mut Formatter<'_>, fields: &[(&dyn Display, &dyn Display)]) -> FmtResult {
    StructShow::new(f, Alternate::Inherit)
        .fields(fields)
        .finish()
}

/// Performs the whole struct output routine from creation of StructShow examplar to finishing.
/// Works with iterator, always inherits alternate mode.
pub fn display_struct_from_iter<'c, K, V, I>(f: &mut Formatter<'_>, fields: I) -> FmtResult
where
    K: Display + 'c,
    V: Display + 'c,
    I: Iterator<Item = (K, V)> + 'c,
{
    StructShow::new(f, Alternate::Inherit)
        .fields_from_iter(fields)
        .finish()
}

type ListEntrier<'t> = &'t dyn Fn(&mut DebugList<'_, '_>, &dyn Display);

/// Lets to output some listed data regarding the propagated value of output alternativeness.
pub struct ListShow<'a, 'b> {
    wrapper: DebugList<'a, 'b>,
    entrier: ListEntrier<'static>,
}

impl<'a, 'b> ListShow<'a, 'b> {
    const USUAL_ENTRIER: ListEntrier<'static> = &|w, v| {
        w.entry(&format_args!("{}", v));
    };
    const ALT_ENTRIER: ListEntrier<'static> = &|w, v| {
        w.entry(&format_args!("{:#}", v));
    };
    const NULL_ENTRIER: ListEntrier<'static> = &|_w, _v| {};

    /// Creates one ListShow examplar starting its output.
    pub fn new(formatter: &'a mut Formatter<'b>, alternate: Alternate) -> Self {
        let entrier = match alternate {
            Alternate::OneLine => Self::USUAL_ENTRIER,
            Alternate::Pretty => Self::ALT_ENTRIER,
            Alternate::Inherit => match formatter.alternate() {
                true => Self::ALT_ENTRIER,
                false => Self::USUAL_ENTRIER,
            },
        };
        Self {
            wrapper: formatter.debug_list(),
            entrier,
        }
    }

    /// Adds one item to the list output.
    pub fn item(&mut self, val: &dyn Display) -> &mut Self {
        (self.entrier)(&mut self.wrapper, val);
        self
    }

    /// Adds one optional item to the list output if its value matches Some(_).
    pub fn item_opt<T: Display>(&mut self, val: &Option<T>) -> &mut Self {
        if let Some(actual_value) = val {
            self.item(actual_value);
        }
        self
    }

    /// Finishes the list output, returning the result.
    pub fn finish(&mut self) -> FmtResult {
        self.entrier = Self::NULL_ENTRIER;
        self.wrapper.finish()
    }

    /// Adds several items to the list output from slice.
    pub fn items(&mut self, items: &[&dyn Display]) -> &mut Self {
        self.items_from_iter(items.iter())
    }

    /// Adds several items to the struct output from iterator.
    pub fn items_from_iter<'c, T, I>(&mut self, items: I) -> &mut Self
    where
        T: Display + 'c,
        I: Iterator<Item = T> + 'c,
    {
        items.for_each(|val| (self.entrier)(&mut self.wrapper, &val));
        self
    }
}

/// Performs the whole list output routine from creation of ListShow examplar to finishing.
/// Works with slice, always inherits alternate mode.
pub fn display_list(f: &mut Formatter<'_>, items: &[&dyn Display]) -> FmtResult {
    display_list_from_iter(f, items.iter())
}

/// Performs the whole list output routine from creation of ListShow examplar to finishing.
/// Works with iterator, always inherits alternate mode.
pub fn display_list_from_iter<'c, T, I>(f: &mut Formatter<'_>, items: I) -> FmtResult
where
    T: Display + 'c,
    I: Iterator<Item = T> + 'c,
{
    ListShow::new(f, Alternate::Inherit)
        .items_from_iter(items)
        .finish()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Debug)]
    struct Integer(isize);

    impl Display for Integer {
        fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
            if f.alternate() {
                f.write_fmt(format_args!("Integer value '{}'", self.0))
            } else {
                f.write_fmt(format_args!("'{}'", self.0))
            }
        }
    }

    #[derive(Debug)]
    struct Complex {
        r: Integer,
        i: Integer,
    }

    impl Complex {
        fn new(r: isize, i: isize) -> Self {
            Self {
                r: Integer(r),
                i: Integer(i),
            }
        }
    }

    impl Display for Complex {
        fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
            display_struct(f, &[(&"r", &self.r), (&'i', &self.i)])
        }
    }

    struct Diverse {
        a: i8,
        b: char,
        c: usize,
        d: Option<&'static str>,
        e: String,
        f: Option<Integer>,
        g: Complex,
    }

    impl Display for Diverse {
        fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
            StructShow::new(f, Alternate::Inherit)
                .fields(&[(&"a", &self.a), (&'b', &self.b), (&"c".to_owned(), &self.c)])
                .field_opt(&"d", &self.d)
                .field(&'e', &self.e)
                .field_opt(&"f".to_owned(), &self.f)
                .field(&"g", &self.g)
                .finish()
        }
    }

    struct Array4 {
        one: Integer,
        two: isize,
        three: char,
        four: Option<char>,
    }

    impl Display for Array4 {
        fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
            ListShow::new(f, Alternate::Inherit)
                .items(&[&self.one, &self.two, &self.three])
                .item_opt(&self.four)
                .finish()
        }
    }

    struct Hector(Vec<isize>);

    impl Display for Hector {
        fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
            display_list_from_iter(f, self.0.iter())
        }
    }

    struct Shmap(std::collections::BTreeMap<String, isize>);

    impl Display for Shmap {
        fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
            display_struct_from_iter(f, self.0.iter())
        }
    }

    #[test]
    fn display() {
        assert_eq!("key: value", &format!("{}", Field::new("key", "value")));
        assert_eq!("key: 12345", &format!("{}", Field::new("key", &12345)));
        assert_eq!(
            "coord: '-43'",
            &format!("{}", Field::new("coord", &Integer(-43)))
        );
        assert_eq!(
            "point: {r: '1', i: '2'}",
            &format!("{}", Field::new("point", &Complex::new(1, 2)))
        );
        assert_eq!(
            "{a: -1, b: z, c: 123456789, d: static string literal, e: Some text, f: '-19', g: {r: '-3', i: '4'}}",
            &format!("{}", Diverse{
                a: -1,
                b: 'z',
                c: 123456789,
                d: Some("static string literal"),
                e: "Some text".to_string(),
                f: Some(Integer(-19)),
                g: Complex::new(-3, 4)
            })
        );
        assert_eq!(
            "{a: -1, b: z, c: 123456789, e: Some text, g: {r: '-3', i: '4'}}",
            &format!(
                "{}",
                Diverse {
                    a: -1,
                    b: 'z',
                    c: 123456789,
                    d: None,
                    e: "Some text".to_string(),
                    f: None,
                    g: Complex::new(-3, 4)
                }
            )
        );
        assert_eq!(
            "['1', 2, c, s]",
            &format!(
                "{}",
                Array4 {
                    one: Integer(1),
                    two: 2,
                    three: 'c',
                    four: Some('s'),
                }
            )
        );
        assert_eq!(
            "['3', 4, d]",
            &format!(
                "{}",
                Array4 {
                    one: Integer(3),
                    two: 4,
                    three: 'd',
                    four: None,
                }
            )
        );
        assert_eq!(
            "[1, 2, 3, 4, 5]",
            &format!("{}", Hector((1..6).into_iter().collect()))
        );
        assert_eq!(
            "{0: 0, 1: 2, 3: 5}",
            &format!(
                "{}",
                Shmap(maplit::btreemap! {
                    "0".into() => 0,
                    "1".into() => 2,
                    "3".into() => 5,
                })
            )
        )
    }

    #[test]
    fn display_alternative() {
        assert_eq!("key: value", &format!("{:#}", Field::new("key", "value")));
        assert_eq!("key: 12345", &format!("{:#}", Field::new("key", &12345)));
        assert_eq!(
            "coord: Integer value '-43'",
            &format!("{:#}", Field::new("coord", &Integer(-43)))
        );
        let point_output = r#"point: {
    r: Integer value '1',
    i: Integer value '2',
}"#;
        assert_eq!(
            point_output,
            &format!("{:#}", Field::new("point", &Complex::new(1, 2)))
        );
        assert_eq!(
            r#"{
    a: -1,
    b: z,
    c: 123456789,
    d: static string literal,
    e: Some text,
    f: Integer value '-19',
    g: {
        r: Integer value '-3',
        i: Integer value '4',
    },
}"#,
            &format!(
                "{:#}",
                Diverse {
                    a: -1,
                    b: 'z',
                    c: 123456789,
                    d: Some("static string literal"),
                    e: "Some text".to_string(),
                    f: Some(Integer(-19)),
                    g: Complex::new(-3, 4)
                }
            )
        );
        assert_eq!(
            r#"{
    a: -1,
    b: z,
    c: 123456789,
    e: Some text,
    g: {
        r: Integer value '-3',
        i: Integer value '4',
    },
}"#,
            &format!(
                "{:#}",
                Diverse {
                    a: -1,
                    b: 'z',
                    c: 123456789,
                    d: None,
                    e: "Some text".to_string(),
                    f: None,
                    g: Complex::new(-3, 4)
                }
            )
        );
        assert_eq!(
            r#"[
    Integer value '5',
    6,
    e,
]"#,
            &format!(
                "{:#}",
                Array4 {
                    one: Integer(5),
                    two: 6,
                    three: 'e',
                    four: None,
                }
            )
        );
        assert_eq!(
            r#"[
    Integer value '7',
    8,
    f,
    g,
]"#,
            &format!(
                "{:#}",
                Array4 {
                    one: Integer(7),
                    two: 8,
                    three: 'f',
                    four: Some('g'),
                }
            )
        );
        assert_eq!(
            r#"[
    1,
    2,
    3,
    4,
    5,
]"#,
            &format!("{:#}", Hector((1..6).into_iter().collect()))
        );
        assert_eq!(
            r#"{
    0: 0,
    1: 2,
    3: 5,
}"#,
            &format!(
                "{:#}",
                Shmap(maplit::btreemap! {
                    "0".into() => 0,
                    "1".into() => 2,
                    "3".into() => 5,
                })
            )
        );
    }

    #[test]
    fn debug() {
        assert_eq!(
            "\"key\": \"value\"",
            &format!("{:?}", Field::new("key", "value"))
        );
        assert_eq!(
            "\"key\": 12345",
            &format!("{:?}", Field::new("key", &12345))
        );
        assert_eq!(
            "\"coord\": Integer(-43)",
            &format!("{:?}", Field::new("coord", &Integer(-43)))
        );
        assert_eq!(
            "\"point\": Complex { r: Integer(1), i: Integer(2) }",
            &format!("{:?}", Field::new("point", &Complex::new(1, 2)))
        );
    }

    #[test]
    fn debug_alternative() {
        assert_eq!(
            "\"key\": \"value\"",
            &format!("{:#?}", Field::new("key", "value"))
        );
        assert_eq!(
            "\"key\": 12345",
            &format!("{:#?}", Field::new("key", &12345))
        );
        assert_eq!(
            r#""coord": Integer(
    -43,
)"#,
            &format!("{:#?}", Field::new("coord", &Integer(-43)))
        );
        let point_output = r#""point": Complex {
    r: Integer(
        1,
    ),
    i: Integer(
        2,
    ),
}"#;
        assert_eq!(
            point_output,
            &format!("{:#?}", Field::new("point", &Complex::new(1, 2)))
        );
    }
}
