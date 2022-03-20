use core::{
    fmt::{Debug, DebugList, DebugSet, Display, Formatter, Result as FmtResult},
    format_args,
};

pub struct Field<'a, K: ?Sized, V: ?Sized> {
    key: &'a K,
    val: &'a V,
}

impl<'a, K: ?Sized, V: ?Sized> Field<'a, K, V> {
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

type StructEntrier<'t> = &'t dyn Fn(&mut DebugSet<'_, '_>, &dyn Display, &dyn Display);

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

    pub fn new(formatter: &'a mut Formatter<'b>) -> Self {
        let entrier = match formatter.alternate() {
            true => Self::ALT_ENTRIER,
            false => Self::USUAL_ENTRIER,
        };
        Self {
            wrapper: formatter.debug_set(),
            entrier,
        }
    }

    pub fn field(&mut self, key: &dyn Display, val: &dyn Display) -> &mut Self {
        (self.entrier)(&mut self.wrapper, key, val);
        self
    }

    pub fn field_opt<T: Display>(&mut self, key: &dyn Display, val: &Option<T>) -> &mut Self {
        if let Some(actual_value) = val {
            self.field(key, actual_value);
        }
        self
    }

    pub fn finish(&mut self) -> FmtResult {
        self.entrier = Self::NULL_ENTRIER;
        self.wrapper.finish()
    }

    pub fn fields(&mut self, fields: &[(&dyn Display, &dyn Display)]) -> &mut Self {
        fields.iter().for_each(|(key, val)| {
            (self.entrier)(&mut self.wrapper, key, val);
        });
        self
    }
}

pub fn display_struct(f: &mut Formatter<'_>, fields: &[(&dyn Display, &dyn Display)]) -> FmtResult {
    StructShow::new(f).fields(fields).finish()
}

type ListEntrier<'t> = &'t dyn Fn(&mut DebugList<'_, '_>, &dyn Display);

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

    pub fn new(formatter: &'a mut Formatter<'b>) -> Self {
        let entrier = match formatter.alternate() {
            true => Self::ALT_ENTRIER,
            false => Self::USUAL_ENTRIER,
        };
        Self {
            wrapper: formatter.debug_list(),
            entrier,
        }
    }

    pub fn item(&mut self, val: &dyn Display) -> &mut Self {
        (self.entrier)(&mut self.wrapper, val);
        self
    }

    pub fn item_opt<T: Display>(&mut self, val: &Option<T>) -> &mut Self {
        if let Some(actual_value) = val {
            self.item(actual_value);
        }
        self
    }

    pub fn finish(&mut self) -> FmtResult {
        self.entrier = Self::NULL_ENTRIER;
        self.wrapper.finish()
    }

    pub fn items(&mut self, entries: &[&dyn Display]) -> &mut Self {
        entries.iter().for_each(|val| {
            (self.entrier)(&mut self.wrapper, val);
        });
        self
    }
}

pub fn display_list(f: &mut Formatter<'_>, entries: &[&dyn Display]) -> FmtResult {
    ListShow::new(f).items(entries).finish()
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
            StructShow::new(f)
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
            ListShow::new(f)
                .items(&[&self.one, &self.two, &self.three])
                .item_opt(&self.four)
                .finish()
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
