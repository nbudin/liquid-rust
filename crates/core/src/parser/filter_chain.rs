use std::fmt;
use std::io::Write;
use std::rc::Rc;

use super::Filter;
use crate::error::{Result, ResultLiquidExt, ResultLiquidReplaceExt};
use crate::model::{SharedValueView, ValueView};
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
    pub fn evaluate<'s>(&'s self, runtime: &'s dyn Runtime) -> Result<SharedValueView<'s>> {
        // take either the provided value or the value from the provided variable
        let mut entry = self.entry.evaluate(runtime)?;

        // apply all specified filters
        for filter in &self.filters {
            let source = format!("{}", entry.source());
            entry = SharedValueView(Rc::new(
                filter
                    .evaluate(entry, runtime)
                    .trace("Filter error")
                    .context_key("filter")
                    .value_with(|| format!("{}", filter).into())
                    .context_key("input")
                    .value(source)?,
            ));
        }

        Ok(entry)
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
