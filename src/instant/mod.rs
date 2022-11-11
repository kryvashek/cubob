//! When your struct has field which is some kind of list (Vec, slice, array, etc.) or map (HashMap, BTreeMap, etc.) you
//! can experience additional complications while trying to output them if they have no standard Display implementation.
//! Since this whole crate is aiming on simplification of Display implementations routine, the need in some related solution
//! becomes obvious. And here it is - this module contain trait and generics which can simplify displaying of lists and maps.
//!
//! Usage example:
//! ```
//! use core::fmt::{Display, Formatter, Result as FmtResult};
//! use std::{vec::Vec, collections::HashMap};
//! use cubob::{display_struct, Alternate, StructShow, InstantList, InstantStruct};
//!
//! struct Object {
//!     title: String,
//!     description: Option<String>,
//!     properties: HashMap<String, String>,
//! }
//!
//! struct Space {
//!     tags: Vec<String>,
//!     members: HashMap<usize, Object>,
//! }
//!
//! impl Display for Object {
//!     fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
//!         StructShow::inherit(f)
//!             .field(&"title", &self.title)
//!             .field_opt(&"description", &self.description)
//!             // self.properties field can be displayed as struct without any self-made helpers
//!             .field(&"properties", &InstantStruct::inherit(&self.properties))
//!             .finish()
//!     }
//! }
//!
//! impl Display for Space {
//!     fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
//!         StructShow::inherit(f)
//!             // self.tags field can be displayed as list without any self-made helpers
//!             .field_override(&"tags", &InstantList::inherit(&self.tags), Alternate::OneLine)
//!             // self.members field can be displayed as struct since Object provides Display implementation
//!             .field(&"members", &InstantStruct::inherit(&self.members))
//!             .finish()
//!     }
//! }
//!
//! let space = Space {
//!     tags: vec!["full".into(), "hilbert".into()],
//!     members: {
//!         let mut map = HashMap::new();
//!         map.insert(1, Object {
//!             title: "Ball".into(),
//!             description: Some("The ball Masha had been playing with".into()),
//!             properties: {
//!                 let mut map = HashMap::new();
//!                 map.insert("colour".into(), "green".into());
//!                 map.insert("owner".into(), "masha".into());
//!                 map
//!             }
//!         });
//!         map.insert(2, Object {
//!             title: "Cube".into(),
//!             description: None,
//!             properties: {
//!                 let mut map = HashMap::new();
//!                 map.insert("colour".into(), "red".into());
//!                 map
//!             }
//!         });
//!         map
//!     }
//! };
//! println!("One-line: {}", space);
//! println!("Prettified: {:#}", space);
//! ```

#[cfg(any(feature = "list", feature = "struct"))]
mod iterable;
#[cfg(feature = "list")]
mod list;
#[cfg(feature = "struct")]
mod r#struct;

#[cfg(any(feature = "list", feature = "struct"))]
#[cfg_attr(docsrs, doc(cfg(any(feature = "list", feature = "struct"))))]
pub use iterable::*;
#[cfg(feature = "list")]
#[cfg_attr(docsrs, doc(cfg(feature = "list")))]
pub use list::*;
#[cfg(feature = "struct")]
#[cfg_attr(docsrs, doc(cfg(feature = "struct")))]
pub use r#struct::*;
