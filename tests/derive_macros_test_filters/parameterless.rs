use liquid_core::Result;
use liquid_core::Runtime;
use liquid_core::Value;
use liquid_core::ValueCow;
use liquid_core::{Display_filter, Filter, FilterReflection, ParseFilter};

#[derive(Clone, ParseFilter, FilterReflection)]
#[filter(
    name = "no_args",
    description = "Filter with no arguments.",
    parsed(TestParameterlessFilter)
)]
pub struct TestParameterlessFilterParser;

#[derive(Debug, Default, Display_filter)]
#[name = "no_args"]
pub struct TestParameterlessFilter;

impl Filter for TestParameterlessFilter {
    fn evaluate(&self, _input: ValueCow, _runtime: &dyn Runtime) -> Result<ValueCow> {
        let result = "<>";

        Ok(Value::scalar(result).into())
    }
}
