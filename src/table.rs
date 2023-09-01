use core::fmt::{Display, Formatter, Result as FmtResult};
use std::fmt::Write;

/// Trait which should be implemented for struct to be displayed as table row.
pub trait Row {
    /// List of columns names to be displayed.
    const KEYS: &'static [&'static str];

    /// Returns reference to structs field by index of its name in [`Self::KEYS`].
    /// [`None`] stands for field value that shouldn't be displayed.
    fn value(&self, idx: usize) -> Option<&dyn Display>;
}

/// Type of temporary values to be displayed as table.
/// Examplars of this type are returned from [`AsTable::as_table`].
#[derive(Clone, Copy, Debug)]
pub struct Table<'a, T: ?Sized>(&'a T);

/// Trait for any type which examplar should have an opportunity to be displayed as table.
pub trait AsTable {
    fn as_table(&self) -> Table<'_, Self>;
}

/// Default implementation of displaying as table for any type
/// which can be iterated over examplars of type implementing [`Row`].
impl<T, R> Display for Table<'_, T>
where
    R: Row,
    for<'a> &'a T: IntoIterator<Item = &'a R>,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        let mut rows = self.0.into_iter();

        let Some(mut row) = rows.next() else {
            return f.write_str("[]");
        };

        let sizes = sizes_list(self.0.into_iter());

        f.write_str("\n")?;

        row_fmt(keys_cells::<R>(&sizes), f)?;

        line_fmt(&sizes, f)?;

        while let Some(next_row) = rows.next() {
            row_fmt(values_cells(row, &sizes), f)?;
            row = next_row;
        }

        row_fmt(values_cells(row, &sizes), f)
    }
}

impl<T, R> AsTable for T
where
    R: Row,
    for<'a> &'a T: IntoIterator<Item = &'a R>,
{
    fn as_table(&self) -> Table<'_, Self> {
        Table(self)
    }
}

trait AsDisplay {
    fn as_display(&self) -> &dyn Display;
}

impl AsDisplay for &dyn Display {
    fn as_display(&self) -> &dyn Display {
        *self
    }
}

impl AsDisplay for &str {
    fn as_display(&self) -> &dyn Display {
        self
    }
}

#[derive(Clone, Debug)]
struct Cell<T> {
    value: Option<T>,
    width: usize,
}

impl<T: AsDisplay> Display for Cell<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        let value = self
            .value
            .as_ref()
            .map(AsDisplay::as_display)
            .unwrap_or("".as_display());
        write!(f, "{value:<width$}", width = self.width)
    }
}

impl<T> Cell<T> {
    fn new<V>(value: V, width: usize) -> Self
    where
        V: Into<Option<T>>,
    {
        Self {
            value: value.into(),
            width,
        }
    }
}

impl<T, V> From<(V, usize)> for Cell<T>
where
    V: Into<Option<T>>,
{
    fn from((value, width): (V, usize)) -> Self {
        Cell::new(value, width)
    }
}

fn row_fmt<I, D>(mut cells: I, f: &mut Formatter<'_>) -> FmtResult
where
    I: Iterator<Item = Cell<D>>,
    D: AsDisplay,
{
    let Some(mut cell) = cells.next() else {
        return Ok(());
    };

    while let Some(next_cell) = cells.next() {
        write!(f, "{cell} | ")?;
        cell = next_cell;
    }

    write!(f, "{cell}\n")
}

fn line_fmt(sizes: &Vec<usize>, f: &mut Formatter<'_>) -> FmtResult {
    let mut sizes = sizes.iter();

    let Some(mut width) = sizes.next() else {
        return Ok(());
    };

    while let Some(next_width) = sizes.next() {
        write!(f, "{:-<width$}-+-", "")?;
        width = next_width;
    }

    write!(f, "{:-<width$}\n", "")
}

fn keys_iter<R: Row>() -> impl Iterator<Item = &'static str> + 'static {
    R::KEYS.iter().copied()
}

fn keys_cells<'a, R: Row>(sizes: &'a Vec<usize>) -> impl Iterator<Item = Cell<&'static str>> + 'a {
    keys_iter::<R>().zip(sizes.iter().copied()).map(Cell::from)
}

fn values_iter<'a, R: Row + 'a>(row: &'a R) -> impl Iterator<Item = Option<&'a dyn Display>> + 'a {
    (0..R::KEYS.len()).map(move |idx| row.value(idx))
}

fn values_cells<'a, R: Row + 'a>(
    row: &'a R,
    sizes: &'a Vec<usize>,
) -> impl Iterator<Item = Cell<&'a dyn Display>> + 'a {
    values_iter(row).zip(sizes.iter().copied()).map(Cell::from)
}

fn max_in_place((overall, current): (&mut usize, usize)) {
    if *overall < current {
        *overall = current;
    }
}

fn sizes_list<'a, I, R>(rows: I) -> Vec<usize>
where
    R: Row + 'a,
    I: Iterator<Item = &'a R>,
{
    let mut overall_sizes: Vec<usize> = vec![0; R::KEYS.len()];

    for row in rows {
        let current_sizes = values_iter(row).map(|x| {
            x.map(|v| {
                let mut wc = WriteCounter::new();
                write!(&mut wc, "{v}")
                    .expect("This buffer (WriteCounter) doesn't even need to (re)allocate memory");
                wc.value()
            })
            .unwrap_or(0)
        });
        overall_sizes
            .iter_mut()
            .zip(current_sizes)
            .for_each(max_in_place);
    }

    let keys_sizes = keys_iter::<R>().map(|x| x.len());
    overall_sizes
        .iter_mut()
        .zip(keys_sizes)
        .for_each(max_in_place);

    overall_sizes
}

#[derive(Clone, Copy, Debug)]
struct WriteCounter(usize);

impl WriteCounter {
    fn new() -> Self {
        Self(0)
    }

    fn value(&self) -> usize {
        self.0
    }
}

impl Write for WriteCounter {
    fn write_str(&mut self, s: &str) -> FmtResult {
        self.0 += s.chars().count();
        Ok(())
    }

    fn write_char(&mut self, _: char) -> FmtResult {
        self.0 += 1;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn cell_str() {
        let cell = Cell::new("str", 3);
        let text = format!("{cell}");
        assert_eq!("str", text);
    }

    #[test]
    fn cell_no_value() {
        let cell = Cell::<&'static str>::new(None, 10);
        let text = format!("{cell}");
        assert_eq!("          ", text);
    }

    #[test]
    fn cell_some_int() {
        let value = 5usize;
        let cell = Cell::<&dyn Display>::from((Some(&value as &dyn Display), 10));
        let text = format!("{cell}");
        assert_eq!("5         ", text);
    }

    #[test]
    fn cell_some_point() {
        struct Point(u8, u8);

        impl Display for Point {
            fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
                write!(f, "(x={}; y={})", self.0, self.1)
            }
        }

        let value = Point(1, 2);
        let cell = Cell::<&dyn Display>::from((Some(&value as &dyn Display), 10));
        let text = format!("{cell}");
        assert_eq!("(x=1; y=2)", text);
    }

    fn keys_and_sizes_to_cells<'a>(
        items: &'a [&'static str],
        sizes: &'a [usize],
    ) -> impl Iterator<Item = Cell<&'static str>> + 'a {
        items
            .iter()
            .copied()
            .zip(sizes.iter().copied())
            .map(Cell::from)
    }

    #[test]
    fn keys_row_empty() {
        struct RowFiction;

        impl Display for RowFiction {
            fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
                row_fmt(keys_and_sizes_to_cells(&[], &[15]), f)
            }
        }

        let text = format!("{RowFiction}");
        assert_eq!("", text);
    }

    #[test]
    fn keys_row_single() {
        struct RowFiction;

        impl Display for RowFiction {
            fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
                row_fmt(keys_and_sizes_to_cells(&["single value"], &[15]), f)
            }
        }

        let text = format!("{RowFiction}");
        assert_eq!("single value   \n", text);
    }

    #[test]
    fn keys_row_multiple() {
        struct RowFiction;

        impl Display for RowFiction {
            fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
                row_fmt(keys_and_sizes_to_cells(&["one", "two"], &[5, 6]), f)
            }
        }

        let text = format!("{RowFiction}");
        assert_eq!("one   | two   \n", text);
    }

    fn values_and_sizes_to_cells<'a, 'b>(
        values: &'a [Option<&'b dyn Display>],
        sizes: &'a [usize],
    ) -> impl Iterator<Item = Cell<&'b dyn Display>> + 'a {
        values
            .iter()
            .copied()
            .zip(sizes.iter().copied())
            .map(Cell::from)
    }

    #[test]
    fn values_row_empty() {
        struct RowFiction;

        impl Display for RowFiction {
            fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
                row_fmt(values_and_sizes_to_cells(&[], &[10]), f)
            }
        }

        let text = format!("{RowFiction}");
        assert_eq!("", text);
    }

    #[test]
    fn values_row_single_str() {
        struct RowFiction;

        impl Display for RowFiction {
            fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
                row_fmt(values_and_sizes_to_cells(&[Some(&"value")], &[10]), f)
            }
        }

        let text = format!("{RowFiction}");
        assert_eq!("value     \n", text);
    }

    #[test]
    fn values_row_single_int() {
        struct RowFiction;

        impl Display for RowFiction {
            fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
                row_fmt(values_and_sizes_to_cells(&[Some(&5)], &[10]), f)
            }
        }

        let text = format!("{RowFiction}");
        assert_eq!("5         \n", text);
    }

    #[test]
    fn values_row_single_bool() {
        struct RowFiction;

        impl Display for RowFiction {
            fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
                row_fmt(values_and_sizes_to_cells(&[Some(&true)], &[10]), f)
            }
        }

        let text = format!("{RowFiction}");
        assert_eq!("true      \n", text);
    }

    #[test]
    fn values_row_multiple() {
        struct RowFiction;

        impl Display for RowFiction {
            fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
                row_fmt(
                    values_and_sizes_to_cells(&[Some(&"one"), Some(&"two")], &[10, 5]),
                    f,
                )
            }
        }

        let text = format!("{RowFiction}");
        assert_eq!("one        | two  \n", text);
    }

    impl Row for [Option<i32>; 4] {
        const KEYS: &'static [&'static str] = &["A", "B", "C", "D"];

        fn value(&self, idx: usize) -> Option<&dyn Display> {
            self.get(idx)
                .map(|x| x.as_ref())
                .flatten()
                .map(|x| x as &dyn Display)
        }
    }

    #[test]
    fn sizes() {
        let table = vec![
            [Some(1), Some(2), None, Some(4)],
            [Some(4), None, None, Some(10)],
            [Some(9), Some(12), None, None],
            [None, Some(20), None, Some(28)],
        ];

        let sizes = sizes_list(table.iter());
        let example = [
            1, // all values in column and its name have size less or equal to 1
            2, // all values in column and its name have size less or equal to 2
            1, // all values in column are missing, but key has size 1
            2, // all values in column and its name have size less or equal to 2
        ];
        assert_eq!(&example[..], &sizes);
    }

    #[test]
    fn table() {
        let table = vec![
            [None, Some(2), Some(3), Some(4)],
            [None, None, Some(8), Some(10)],
            [None, Some(12), None, Some(18)],
            [None, Some(20), Some(24), Some(28)],
        ];

        let text = table.as_table().to_string();
        let example = r#"
A | B  | C  | D 
--+----+----+---
  | 2  | 3  | 4 
  |    | 8  | 10
  | 12 |    | 18
  | 20 | 24 | 28
"#;
        assert_eq!(example, text);
    }
}
