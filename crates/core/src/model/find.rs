//! Find a `ValueView` nested in an `ObjectView`

use std::fmt;
use std::rc::Rc;
use std::slice;

use crate::error::Result;

use super::ScalarCow;
use super::Value;
use super::ValueCow;
use super::ValueView;

/// Path to a value in an `Object`.
///
/// There is guaranteed always at least one element.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Path<'s>(Vec<ScalarCow<'s>>);

impl<'s> Path<'s> {
    /// Create a `Value` reference.
    pub fn with_index<I: Into<ScalarCow<'s>>>(value: I) -> Self {
        let indexes = vec![value.into()];
        Path(indexes)
    }

    /// Append an index.
    pub fn push<I: Into<ScalarCow<'s>>>(&mut self, value: I) {
        self.0.push(value.into());
    }

    /// Reserves capacity for at least `additional` more elements to be inserted
    /// in the given `Path`. The `Path` may reserve more space to avoid
    /// frequent reallocations. After calling `reserve`, capacity will be
    /// greater than or equal to `self.len() + additional`. Does nothing if
    /// capacity is already sufficient.
    pub fn reserve(&mut self, additional: usize) {
        self.0.reserve(additional);
    }

    /// Access the `Value` reference.
    pub fn iter(&self) -> PathIter<'_, '_> {
        PathIter(self.0.iter())
    }

    /// Extracts a slice containing the entire vector.
    #[inline]
    pub fn as_slice(&self) -> &[ScalarCow<'s>] {
        self.0.as_slice()
    }
}

impl<'s> Extend<ScalarCow<'s>> for Path<'s> {
    fn extend<T: IntoIterator<Item = ScalarCow<'s>>>(&mut self, iter: T) {
        self.0.extend(iter);
    }
}

impl<'s> ::std::ops::Deref for Path<'s> {
    type Target = [ScalarCow<'s>];

    #[inline]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<'s> ::std::borrow::Borrow<[ScalarCow<'s>]> for Path<'s> {
    #[inline]
    fn borrow(&self) -> &[ScalarCow<'s>] {
        self
    }
}

impl<'s> AsRef<[ScalarCow<'s>]> for Path<'s> {
    #[inline]
    fn as_ref(&self) -> &[ScalarCow<'s>] {
        self
    }
}

impl<'s> fmt::Display for Path<'s> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let data = itertools::join(self.iter().map(ValueView::render), ".");
        write!(f, "{}", data)
    }
}

/// Iterate over indexes in a `Value`'s `Path`.
#[derive(Debug)]
pub struct PathIter<'i, 's>(slice::Iter<'i, ScalarCow<'s>>);

impl<'i, 's: 'i> Iterator for PathIter<'i, 's> {
    type Item = &'i ScalarCow<'s>;

    #[inline]
    fn next(&mut self) -> Option<&'i ScalarCow<'s>> {
        self.0.next()
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        self.0.size_hint()
    }

    #[inline]
    fn count(self) -> usize {
        self.0.count()
    }
}

impl<'i, 's: 'i> ExactSizeIterator for PathIter<'i, 's> {
    #[inline]
    fn len(&self) -> usize {
        self.0.len()
    }
}

/// Find a `ValueView` nested in an `ObjectView`
pub fn try_find<'o>(value: &'o dyn ValueView, path: &[ScalarCow<'_>]) -> Option<ValueCow<'o>> {
    let mut indexes = path.iter();
    let mut current = ValueCow::Shared(Rc::new(value));
    loop {
        let index = match indexes.next() {
            Some(index) => index,
            None => {
                return Some(current);
            }
        };
        current = augmented_get(value, index)?;
    }
}

fn augmented_get<'o>(
    value: &'o (dyn ValueView + 'o),
    index: &ScalarCow<'_>,
) -> Option<ValueCow<'o>> {
    if let Some(arr) = value.as_array() {
        if let Some(index) = index.to_integer_strict() {
            arr.get(index)
        } else {
            match &*index.to_kstr() {
                "first" => arr.first(),
                "last" => arr.last(),
                "size" => Some(ValueCow::Owned(Value::scalar(arr.size()))),
                _ => None,
            }
        }
    } else if let Some(obj) = value.as_object() {
        let index = index.to_kstr();
        obj.get(index.as_str()).or_else(|| match index.as_str() {
            "size" => Some(ValueCow::Owned(Value::scalar(obj.size()))),
            _ => None,
        })
    } else if let Some(scalar) = value.as_scalar() {
        let index = index.to_kstr();
        match index.as_str() {
            "size" => Some(ValueCow::Owned(Value::scalar(
                scalar.to_kstr().as_str().len() as i64,
            ))),
            _ => None,
        }
    } else {
        None
    }
}

/// Find a `ValueView` nested in an `ObjectView`
pub fn find<'o>(value: &'o dyn ValueView, path: &[ScalarCow<'_>]) -> Result<ValueCow<'o>> {
    if let Some(res) = try_find(value, path) {
        Ok(res)
    } else {
        Ok(ValueCow::Owned(Value::Nil))
    }
}
