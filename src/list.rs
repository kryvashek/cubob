use super::Alternate;
use core::{
    fmt::{DebugList, Display, Formatter, Result as FmtResult},
    format_args,
};

type ListEntrier = fn(&mut DebugList<'_, '_>, &dyn Display);

fn usual_list_entrier(w: &mut DebugList, v: &dyn Display) {
    w.entry(&format_args!("{}", v));
}

fn alternative_list_entrier(w: &mut DebugList, v: &dyn Display) {
    w.entry(&format_args!("{:#}", v));
}

fn null_list_entrier(_: &mut DebugList, _: &dyn Display) {}

/// Lets to output some listed data regarding the propagated value of output alternativeness.
pub struct ListShow<'a, 'b> {
    wrapper: DebugList<'a, 'b>,
    entrier: ListEntrier,
    inherited_value: bool,
}

impl<'a, 'b> ListShow<'a, 'b> {
    fn choose_entrier(alternate: Alternate, inherited_value: bool) -> ListEntrier {
        match alternate {
            Alternate::OneLine => usual_list_entrier,
            Alternate::Pretty => alternative_list_entrier,
            Alternate::Inherit => match inherited_value {
                false => usual_list_entrier,
                true => alternative_list_entrier,
            },
        }
    }

    /// Creates one ListShow examplar starting its output.
    pub fn new(formatter: &'a mut Formatter<'b>, alternate: Alternate) -> Self {
        let inherited_value = formatter.alternate();
        let entrier = Self::choose_entrier(alternate, inherited_value);
        Self {
            wrapper: formatter.debug_list(),
            entrier,
            inherited_value,
        }
    }

    /// Adds one item to the list output.
    pub fn item(&mut self, val: &dyn Display) -> &mut Self {
        (self.entrier)(&mut self.wrapper, val);
        self
    }

    /// Adds one item to the list output.
    /// May cause unknown (I just unsure what will happen) behaviour if called after finish().
    pub fn item_override(&mut self, val: &dyn Display, alternate: Alternate) -> &mut Self {
        let entrier = Self::choose_entrier(alternate, self.inherited_value);
        entrier(&mut self.wrapper, val);
        self
    }

    /// Adds one optional item to the list output if its value matches Some(_).
    pub fn item_opt<T: Display>(&mut self, val: &Option<T>) -> &mut Self {
        if let Some(actual_value) = val {
            self.item(actual_value);
        }
        self
    }

    /// Adds one optional item to the list output if its value matches Some(_).
    /// May cause unknown (I just unsure what will happen) behaviour if called after finish().
    pub fn item_opt_override<T: Display>(
        &mut self,
        val: &Option<T>,
        alternate: Alternate,
    ) -> &mut Self {
        if let Some(actual_value) = val {
            self.item_override(actual_value, alternate);
        }
        self
    }

    /// Finishes the list output, returning the result.
    pub fn finish(&mut self) -> FmtResult {
        self.entrier = null_list_entrier;
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
