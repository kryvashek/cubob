use core::{
    fmt::{Debug, Display, Formatter, Result as FmtResult},
    format_args,
};
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