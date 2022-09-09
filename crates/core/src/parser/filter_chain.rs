use std::fmt;
use std::io::Write;

use super::Filter;
use crate::error::{Result, ResultLiquidExt, ResultLiquidReplaceExt};
use crate::model::{ValueCow, ValueView};
use crate::runtime::Expression;
use crate::runtime::Renderable;
use crate::runtime::Runtime;

/// A `Value` expression.
#[derive(Debug)]
pub struct FilterChain {
    entry: Expression,
    filters: Vec<Box<dyn Filter>>,
}

impl FilterChain {
    /// Create a new expression.
    pub fn new(entry: Expression, filters: Vec<Box<dyn Filter>>) -> Self {
        Self { entry, filters }
    }

    /// Process `Value` expression within `runtime`'s stack.
    pub fn evaluate<'s>(&'s self, runtime: &'s dyn Runtime) -> Result<ValueCow<'s>> {
        // take either the provided value or the value from the provided variable
        let entry = self.entry.evaluate(runtime)?;

        Ok(entry)
        // apply all specified filters
        // let mut filter_steps: Vec<ValueCow<'s>> = Vec::with_capacity(self.filters.len());

        // for (index, filter) in self.filters.iter().enumerate() {
        //     let last_value: &'s ValueCow = if index > 0 {
        //         &filter_steps[index - 1]
        //     } else {
        //         &entry
        //     };

        //     let filtered_value = filter
        //         .evaluate(last_value.as_view(), runtime)
        //         .trace("Filter error")
        //         .context_key("filter")
        //         .value_with(|| format!("{}", filter).into())
        //         .context_key("input")
        //         .value_with(|| format!("{}", last_value.source()).into())?;
        //     filter_steps[index] = filtered_value;
        // }

        // let final_value = filter_steps[self.filters.len() - 1].clone();
        // Ok(final_value)
    }
}

impl fmt::Display for FilterChain {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{} | {}",
            self.entry,
            itertools::join(&self.filters, " | ")
        )
    }
}

impl Renderable for FilterChain {
    fn render_to(&self, writer: &mut dyn Write, runtime: &dyn Runtime) -> Result<()> {
        let entry = self.evaluate(runtime)?;
        write!(writer, "{}", entry.render()).replace("Failed to render")?;
        Ok(())
    }
}
