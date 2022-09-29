use std::fmt;
use std::rc::Rc;

use crate::model::KStringCow;

use super::DisplayCow;
use super::State;
use super::Value;
use super::{ValueView, ValueViewCmp};
use crate::model::array::{Array, ArrayView};
use crate::model::object::{Object, ObjectView};
use crate::model::scalar::{Scalar, ScalarCow};

#[derive(Clone, Debug)]
pub struct SharedValueView<'s>(pub Rc<Box<dyn ValueView + 's>>);

impl<'s> SharedValueView<'s> {
    pub fn from_value(value: Value) -> Self {
        SharedValueView(Rc::new(Box::new(value)))
    }

    pub fn from_view(view: &'s (dyn ValueView + 's)) -> Self {
        SharedValueView(Rc::new(Box::new(view)))
    }

    pub fn from_boxed_view(view: Box<dyn ValueView + 's>) -> Self {
        SharedValueView(Rc::new(view))
    }

    /// Extracts the owned data.
    ///
    /// Clones the data if it is not already owned.
    pub fn into_owned(self) -> Value {
        self.0.to_value()
    }

    /// Performs the conversion.
    pub fn as_view(&self) -> &dyn ValueView {
        self.0.as_ref().as_ref()
    }
}

impl<'s> ValueView for SharedValueView<'s> {
    fn as_debug(&self) -> &dyn fmt::Debug {
        self
    }

    fn render(&self) -> DisplayCow<'_> {
        self.as_view().render()
    }
    fn source(&self) -> DisplayCow<'_> {
        self.as_view().source()
    }
    fn type_name(&self) -> &'static str {
        self.as_view().type_name()
    }
    fn query_state(&self, state: State) -> bool {
        self.as_view().query_state(state)
    }

    fn to_kstr(&self) -> KStringCow<'_> {
        self.as_view().to_kstr()
    }
    fn to_value(&self) -> Value {
        self.as_view().to_value()
    }

    fn as_scalar(&self) -> Option<ScalarCow<'_>> {
        self.as_view().as_scalar()
    }

    fn as_array(&self) -> Option<&dyn ArrayView> {
        self.as_view().as_array()
    }

    fn as_object(&self) -> Option<&dyn ObjectView> {
        self.as_view().as_object()
    }

    fn as_state(&self) -> Option<State> {
        self.as_view().as_state()
    }

    fn is_nil(&self) -> bool {
        self.as_view().is_nil()
    }
}

impl From<Value> for SharedValueView<'static> {
    fn from(other: Value) -> Self {
        SharedValueView::from_value(other)
    }
}

impl<'s> From<&'s Value> for SharedValueView<'s> {
    fn from(other: &'s Value) -> Self {
        SharedValueView::from_view(other)
    }
}

impl From<Scalar> for SharedValueView<'static> {
    fn from(other: Scalar) -> Self {
        SharedValueView::from_value(Value::Scalar(other))
    }
}

impl From<Array> for SharedValueView<'static> {
    fn from(other: Array) -> Self {
        SharedValueView::from_value(Value::Array(other))
    }
}

impl From<Object> for SharedValueView<'static> {
    fn from(other: Object) -> Self {
        SharedValueView::from_value(Value::Object(other))
    }
}

impl From<State> for SharedValueView<'static> {
    fn from(other: State) -> Self {
        SharedValueView::from_value(Value::State(other))
    }
}

impl<'s> From<&'s (dyn ValueView + 's)> for SharedValueView<'s> {
    fn from(other: &'s (dyn ValueView + 's)) -> Self {
        SharedValueView::from_view(other)
    }
}

impl<'s> From<Box<dyn ValueView + 's>> for SharedValueView<'s> {
    fn from(other: Box<dyn ValueView + 's>) -> Self {
        SharedValueView::from_boxed_view(other)
    }
}

impl<'v> Default for SharedValueView<'v> {
    fn default() -> Self {
        SharedValueView::from_value(Value::default())
    }
}

impl<'v> PartialEq<Value> for SharedValueView<'v> {
    fn eq(&self, other: &Value) -> bool {
        super::value_eq(self.as_view(), other.as_view())
    }
}

impl<'v> PartialEq<SharedValueView<'v>> for SharedValueView<'v> {
    fn eq(&self, other: &Self) -> bool {
        super::value_eq(self.as_view(), other.as_view())
    }
}

impl<'v> PartialEq<ValueViewCmp<'v>> for SharedValueView<'v> {
    fn eq(&self, other: &ValueViewCmp<'v>) -> bool {
        ValueViewCmp::new(self.as_view()) == *other
    }
}

impl<'v> PartialEq<i64> for SharedValueView<'v> {
    fn eq(&self, other: &i64) -> bool {
        super::value_eq(self.as_view(), other)
    }
}

impl<'v> PartialEq<f64> for SharedValueView<'v> {
    fn eq(&self, other: &f64) -> bool {
        super::value_eq(self.as_view(), other)
    }
}

impl<'v> PartialEq<bool> for SharedValueView<'v> {
    fn eq(&self, other: &bool) -> bool {
        super::value_eq(self.as_view(), other)
    }
}

impl<'v> PartialEq<crate::model::scalar::DateTime> for SharedValueView<'v> {
    fn eq(&self, other: &crate::model::scalar::DateTime) -> bool {
        super::value_eq(self.as_view(), other)
    }
}

impl<'v> PartialEq<crate::model::scalar::Date> for SharedValueView<'v> {
    fn eq(&self, other: &crate::model::scalar::Date) -> bool {
        super::value_eq(self.as_view(), other)
    }
}

impl<'v> PartialEq<str> for SharedValueView<'v> {
    fn eq(&self, other: &str) -> bool {
        let other = KStringCow::from_ref(other);
        super::value_eq(self.as_view(), &other)
    }
}

impl<'v> PartialEq<&'v str> for SharedValueView<'v> {
    fn eq(&self, other: &&str) -> bool {
        super::value_eq(self.as_view(), other)
    }
}

impl<'v> PartialEq<String> for SharedValueView<'v> {
    fn eq(&self, other: &String) -> bool {
        super::value_eq(self.as_view(), other)
    }
}

impl<'v> PartialEq<crate::model::KString> for SharedValueView<'v> {
    fn eq(&self, other: &crate::model::KString) -> bool {
        super::value_eq(self.as_view(), &other.as_ref())
    }
}

impl<'v> PartialEq<crate::model::KStringRef<'v>> for SharedValueView<'v> {
    fn eq(&self, other: &crate::model::KStringRef<'v>) -> bool {
        super::value_eq(self.as_view(), other)
    }
}

impl<'v> PartialEq<crate::model::KStringCow<'v>> for SharedValueView<'v> {
    fn eq(&self, other: &crate::model::KStringCow<'v>) -> bool {
        super::value_eq(self.as_view(), other)
    }
}
