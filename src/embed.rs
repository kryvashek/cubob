#[cfg(feature = "list")]
mod list {
    use crate::{Alternate, ListShow};
    use core::fmt::{Formatter, Result as FmtResult};

    /// Trait letting to define embedding of implementing type list output into other type list output.
    pub trait EmbedList {
        fn embed(&self, show: &mut ListShow);
    }

    impl<'a, 'b> ListShow<'a, 'b> {
        /// Embeds given EmbedList implementing type examplar output into current output.
        pub fn embed<E: EmbedList>(&mut self, embedding: &E) -> &mut Self {
            embedding.embed(self);
            self
        }
    }

    /// Routine to simplify Display implementation for type which already implements EmbedList.
    #[inline]
    pub fn display_list_from_embed<E: EmbedList>(
        this: &E,
        formatter: &mut Formatter<'_>,
        alternate: Alternate,
    ) -> FmtResult {
        ListShow::new(formatter, alternate).embed(this).finish()
    }
}

#[cfg(feature = "struct")]
mod r#struct {
    use crate::{Alternate, StructShow};
    use core::fmt::{Formatter, Result as FmtResult};

    /// Trait letting to define embedding of implementing type struct output into other type struct output.
    pub trait EmbedStruct {
        fn embed(&self, show: &mut StructShow);
    }

    impl<'a, 'b> StructShow<'a, 'b> {
        /// Embeds given EmbedStruct implementing type examplar output into current output.
        pub fn embed<E: EmbedStruct>(&mut self, embedding: &E) -> &mut Self {
            embedding.embed(self);
            self
        }
    }

    /// Routine to simplify Display implementation for type which already implements EmbedStruct.
    #[inline]
    pub fn display_struct_from_embed<E: EmbedStruct>(
        this: &E,
        formatter: &mut Formatter<'_>,
        alternate: Alternate,
    ) -> FmtResult {
        StructShow::new(formatter, alternate).embed(this).finish()
    }
}

#[cfg(feature = "list")]
pub use list::*;

#[cfg(feature = "struct")]
pub use r#struct::*;
