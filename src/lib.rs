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

mod field;
mod r#struct;
mod list;

#[cfg(test)]
mod tests;

pub use field::*;
pub use r#struct::*;
pub use list::*;


/// Alternate mode to use while outputting.
pub enum Alternate {
    /// Output data in one line (matches Formatter::alternate() == false).
    OneLine,
    /// Output data in prettified format (matches Formatter::alternate() == true).
    Pretty,
    /// Output data in format regarding alternate mode of given Formatter examplar.
    Inherit,
}
