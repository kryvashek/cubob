use crate::{Alternate, DisplayPair};
use core::{
    fmt::{DebugSet, Display, Formatter, Result as FmtResult},
    format_args,
};

type StructEntrier = fn(&mut DebugSet<'_, '_>, &dyn Display, &dyn Display);

fn usual_struct_entrier(w: &mut DebugSet, k: &dyn Display, v: &dyn Display) {
    w.entry(&format_args!("{}: {}", k, v));
}

fn alternative_struct_entrier(w: &mut DebugSet, k: &dyn Display, v: &dyn Display) {
    w.entry(&format_args!("{}: {:#}", k, v));
}

fn null_struct_entrier(_: &mut DebugSet, _: &dyn Display, _: &dyn Display) {}

fn inherit_entrier(inherited_value: bool) -> StructEntrier {
    match inherited_value {
        false => usual_struct_entrier,
        true => alternative_struct_entrier,
    }
}

/// Lets to output some structure regarding the propagated value of output alternativeness.
#[cfg_attr(docsrs, doc(cfg(feature = "struct")))]
pub struct StructShow<'a, 'b> {
    wrapper: DebugSet<'a, 'b>,
    entrier: StructEntrier,
    inherited_value: bool,
}

impl<'a, 'b> StructShow<'a, 'b> {
    fn choose_entrier(alternate: Alternate, inherited_value: bool) -> StructEntrier {
        match alternate {
            Alternate::OneLine => usual_struct_entrier,
            Alternate::Pretty => alternative_struct_entrier,
            Alternate::Inherit => inherit_entrier(inherited_value),
        }
    }

    /// Creates one [StructShow] examplar starting its output.
    pub fn new(formatter: &'a mut Formatter<'b>, alternate: Alternate) -> Self {
        let inherited_value = formatter.alternate();
        let entrier = Self::choose_entrier(alternate, inherited_value);
        Self {
            wrapper: formatter.debug_set(),
            entrier,
            inherited_value,
        }
    }

    /// Creates one [StructShow] examplar with [Alternate::Inherit] setting and starts its output.
    pub fn inherit(formatter: &'a mut Formatter<'b>) -> Self {
        let inherited_value = formatter.alternate();
        let entrier = inherit_entrier(inherited_value);
        Self {
            wrapper: formatter.debug_set(),
            entrier,
            inherited_value,
        }
    }

    /// Adds one key-value pair to the struct output.
    pub fn field(&mut self, key: &dyn Display, val: &dyn Display) -> &mut Self {
        (self.entrier)(&mut self.wrapper, key, val);
        self
    }

    /// Adds one key-value pair to the struct output.
    pub fn field_override(
        &mut self,
        key: &dyn Display,
        val: &dyn Display,
        alternate: Alternate,
    ) -> &mut Self {
        // Safety: since only specified subset of predefined functions can take place in self.entrier,
        // and null_struct_entrier is one of them, the comparison through pointer values is safe enough.
        if null_struct_entrier as usize != self.entrier as usize {
            let entrier = Self::choose_entrier(alternate, self.inherited_value);
            entrier(&mut self.wrapper, key, val);
        }
        self
    }

    /// Adds one optional key-value pair to the struct output if its value matches Some(_).
    pub fn field_opt<T: Display>(&mut self, key: &dyn Display, val: &Option<T>) -> &mut Self {
        if let Some(actual_value) = val {
            self.field(key, actual_value);
        }
        self
    }

    /// Adds one optional key-value pair to the struct output if its value matches Some(_).
    pub fn field_opt_override<T: Display>(
        &mut self,
        key: &dyn Display,
        val: &Option<T>,
        alternate: Alternate,
    ) -> &mut Self {
        if let Some(actual_value) = val {
            self.field_override(key, actual_value, alternate);
        }
        self
    }

    /// Finishes the struct output, returning the result.
    pub fn finish(&mut self) -> FmtResult {
        self.entrier = null_struct_entrier;
        self.wrapper.finish()
    }

    /// Adds several key-value pair to the struct output from slice.
    pub fn fields(&mut self, fields: &[(&dyn Display, &dyn Display)]) -> &mut Self {
        self.fields_from_iter(fields.iter())
    }

    /// Adds several key-value pair to the struct output from iterator.
    pub fn fields_from_iter<'c, I>(&mut self, fields: I) -> &mut Self
    where
        I: Iterator + 'c,
        I::Item: DisplayPair,
    {
        fields.for_each(|p| (self.entrier)(&mut self.wrapper, p.left(), p.rifgt()));
        self
    }

    /// Returns value of `alternate()` of formatter used on struct examplar creation.
    pub fn alternate(&self) -> bool {
        self.inherited_value
    }
}

/// Performs the whole struct output routine from creation of [StructShow] examplar to finishing (for example see the crate-level documentation).
/// Works with slice, always inherits alternate mode.
#[cfg_attr(docsrs, doc(cfg(feature = "struct")))]
pub fn display_struct(f: &mut Formatter<'_>, fields: &[(&dyn Display, &dyn Display)]) -> FmtResult {
    StructShow::new(f, Alternate::Inherit)
        .fields(fields)
        .finish()
}

/// Performs the whole struct output routine from creation of [StructShow] examplar to finishing.
/// Works with iterator, always inherits alternate mode.
#[cfg_attr(docsrs, doc(cfg(feature = "struct")))]
pub fn display_struct_from_iter<'c, I>(f: &mut Formatter<'_>, fields: I) -> FmtResult
where
    I: Iterator + 'c,
    I::Item: DisplayPair,
{
    StructShow::new(f, Alternate::Inherit)
        .fields_from_iter(fields)
        .finish()
}
