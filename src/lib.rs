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
//!let line = Line{ a: Point{ x: 0, y: 0}, b: Point{ x: 1, y: 1} };
//!println!("One-line: {}", line);
//!println!("Prettified: {:#}", line);
//! ```

#![cfg_attr(docsrs, feature(doc_cfg))]

#[cfg(feature = "custom")]
#[cfg_attr(docsrs, doc(cfg(feature = "custom")))]
mod custom;
#[cfg(feature = "embed")]
#[cfg_attr(docsrs, doc(cfg(feature = "embed")))]
mod embed;
#[cfg(feature = "field")]
#[cfg_attr(docsrs, doc(cfg(feature = "field")))]
mod field;
#[cfg(feature = "instant")]
#[cfg_attr(docsrs, doc(cfg(feature = "instant")))]
mod instant;
#[cfg(feature = "list")]
#[cfg_attr(docsrs, doc(cfg(feature = "list")))]
mod list;
#[cfg(feature = "struct")]
#[cfg_attr(docsrs, doc(cfg(feature = "struct")))]
mod pair;
#[cfg(feature = "params")]
#[cfg_attr(docsrs, doc(cfg(feature = "params")))]
mod params;
#[cfg(feature = "path")]
#[cfg_attr(docsrs, doc(cfg(feature = "path")))]
mod path;
#[cfg(feature = "struct")]
#[cfg_attr(docsrs, doc(cfg(feature = "struct")))]
mod r#struct;

#[cfg(all(test, feature = "list", feature = "struct", feature = "field"))]
mod tests;

#[cfg(feature = "custom")]
pub use custom::*;
#[cfg(feature = "embed")]
pub use embed::*;
#[cfg(feature = "field")]
pub use field::*;
#[cfg(feature = "instant")]
pub use instant::*;
#[cfg(feature = "list")]
pub use list::*;
#[cfg(feature = "struct")]
pub use pair::*;
#[cfg(feature = "params")]
pub use params::*;
#[cfg(feature = "path")]
pub use path::*;
#[cfg(feature = "struct")]
pub use r#struct::*;

/// Alternate mode to use while outputting.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Alternate {
    /// Output data in one line (matches [Formatter][core::fmt::Formatter]::alternate() == false).
    OneLine,
    /// Output data in prettified format (matches [Formatter][core::fmt::Formatter]::alternate() == true).
    Pretty,
    /// Output data in format regarding alternate mode of given [Formatter][core::fmt::Formatter] examplar.
    Inherit,
}
