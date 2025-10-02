#[cfg(feature = "list")]
#[cfg_attr(docsrs, doc(cfg(feature = "list")))]
mod list {
    use crate::{Alternate, ListShow};
    use core::fmt::{Formatter, Result as FmtResult};

    /// Trait letting to define embedding of implementing type list output into other type list output.
    #[cfg_attr(docsrs, doc(cfg(all(feature = "embed", feature = "list"))))]
    pub trait EmbedList {
        /// Embed given [EmbedList] implementing type examplar output into the specified [ListShow] output.
        fn embed(&self, show: &mut ListShow);
    }

    impl<'a, 'b> ListShow<'a, 'b> {
        /// Embeds given [EmbedList] implementing type examplar output into current output.
        pub fn embed<E>(&mut self, embedding: &E) -> &mut Self
        where
            E: EmbedList + ?Sized,
        {
            embedding.embed(self);
            self
        }
    }

    /// Routine to simplify [Display][core::fmt::Display] implementation for type which already implements [EmbedList].
    #[cfg_attr(docsrs, doc(cfg(all(feature = "embed", feature = "list"))))]
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
#[cfg_attr(docsrs, doc(cfg(feature = "struct")))]
mod r#struct {
    use crate::{Alternate, StructShow};
    use core::fmt::{Formatter, Result as FmtResult};

    /// Trait letting to define embedding of implementing type struct output into other type struct output.
    #[cfg_attr(docsrs, doc(cfg(all(feature = "embed", feature = "struct"))))]
    pub trait EmbedStruct {
        /// Embed given [EmbedStruct] implementing type examplar output into the specified [StructShow] output.
        fn embed(&self, show: &mut StructShow);
    }

    impl<'a, 'b> StructShow<'a, 'b> {
        /// Embeds given [EmbedStruct] implementing type examplar output into current output.
        pub fn embed<E>(&mut self, embedding: &E) -> &mut Self
        where
            E: EmbedStruct + ?Sized,
        {
            embedding.embed(self);
            self
        }
    }

    /// Routine to simplify [Display][core::fmt::Display] implementation for type which already implements [EmbedStruct].
    #[cfg_attr(docsrs, doc(cfg(all(feature = "embed", feature = "struct"))))]
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
