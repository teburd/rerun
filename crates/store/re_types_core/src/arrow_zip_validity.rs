//! Easily iterate over arrow values that may contain nulls.
//!
//! Code adapted from <https://github.com/jorgecarleitao/arrow2/blob/65c0ac4182748ec39b0a158ec5ebde829aae12b6/src/bitmap/utils/zip_validity.rs>
//! Originally written by [Jorge Leitao](https://github.com/jorgecarleitao),
//! under Apache License 2.0 (see `LICENSE-APACHE` file).
//!
//! Adapted by Rerun to work with `arrow` crate.

use arrow::{buffer::NullBuffer, util::bit_iterator::BitIterator};

/// An [`Iterator`] over validity and values.
#[derive(Debug, Clone)]
pub struct ZipValidityIter<T, I, V>
where
    I: Iterator<Item = T>,
    V: Iterator<Item = bool>,
{
    values: I,
    validity: V,
}

impl<T, I, V> ZipValidityIter<T, I, V>
where
    I: Iterator<Item = T>,
    V: Iterator<Item = bool>,
{
    /// Creates a new [`ZipValidityIter`].
    /// # Panics
    /// This function panics if the `size_hints` of the iterators are different
    pub fn new(values: I, validity: V) -> Self {
        assert_eq!(values.size_hint(), validity.size_hint());
        Self { values, validity }
    }
}

impl<T, I, V> Iterator for ZipValidityIter<T, I, V>
where
    I: Iterator<Item = T>,
    V: Iterator<Item = bool>,
{
    type Item = Option<T>;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        let value = self.values.next();
        let is_valid = self.validity.next();
        is_valid
            .zip(value)
            .map(|(is_valid, value)| is_valid.then_some(value))
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        self.values.size_hint()
    }

    #[inline]
    fn nth(&mut self, n: usize) -> Option<Self::Item> {
        let value = self.values.nth(n);
        let is_valid = self.validity.nth(n);
        is_valid
            .zip(value)
            .map(|(is_valid, value)| is_valid.then_some(value))
    }
}

impl<T, I, V> DoubleEndedIterator for ZipValidityIter<T, I, V>
where
    I: DoubleEndedIterator<Item = T>,
    V: DoubleEndedIterator<Item = bool>,
{
    #[inline]
    fn next_back(&mut self) -> Option<Self::Item> {
        let value = self.values.next_back();
        let is_valid = self.validity.next_back();
        is_valid
            .zip(value)
            .map(|(is_valid, value)| is_valid.then_some(value))
    }
}

impl<T, I, V> ExactSizeIterator for ZipValidityIter<T, I, V>
where
    I: ExactSizeIterator<Item = T>,
    V: ExactSizeIterator<Item = bool>,
{
}

/// An [`Iterator`] over [`Option<T>`]
/// This enum can be used in two distinct ways:
/// * as an iterator, via `Iterator::next`
/// * as an enum of two iterators, via `match self`
///
/// The latter allows specializalizing to when there are no nulls
#[derive(Debug, Clone)]
pub enum ZipValidity<T, I, V>
where
    I: Iterator<Item = T>,
    V: Iterator<Item = bool>,
{
    /// There are no null values
    Required(I),

    /// There are null values
    Optional(ZipValidityIter<T, I, V>),
}

impl<T, I, V> ZipValidity<T, I, V>
where
    I: Iterator<Item = T>,
    V: Iterator<Item = bool>,
{
    /// Returns a new [`ZipValidity`]
    pub fn new(values: I, validity: Option<V>) -> Self {
        match validity {
            Some(validity) => Self::Optional(ZipValidityIter::new(values, validity)),
            _ => Self::Required(values),
        }
    }
}

impl<'a, T, I> ZipValidity<T, I, BitIterator<'a>>
where
    I: Iterator<Item = T>,
{
    /// Returns a new [`ZipValidity`] and drops the `validity` if all values
    /// are valid.
    pub fn new_with_validity(values: I, validity: Option<&'a NullBuffer>) -> Self {
        // only if the validity has nulls we take the optional branch.
        match validity.and_then(|validity| (validity.null_count() > 0).then(|| validity.iter())) {
            Some(validity) => Self::Optional(ZipValidityIter::new(values, validity)),
            _ => Self::Required(values),
        }
    }
}

impl<T, I, V> Iterator for ZipValidity<T, I, V>
where
    I: Iterator<Item = T>,
    V: Iterator<Item = bool>,
{
    type Item = Option<T>;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        match self {
            Self::Required(values) => values.next().map(Some),
            Self::Optional(zipped) => zipped.next(),
        }
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        match self {
            Self::Required(values) => values.size_hint(),
            Self::Optional(zipped) => zipped.size_hint(),
        }
    }

    #[inline]
    fn nth(&mut self, n: usize) -> Option<Self::Item> {
        match self {
            Self::Required(values) => values.nth(n).map(Some),
            Self::Optional(zipped) => zipped.nth(n),
        }
    }
}

impl<T, I, V> DoubleEndedIterator for ZipValidity<T, I, V>
where
    I: DoubleEndedIterator<Item = T>,
    V: DoubleEndedIterator<Item = bool>,
{
    #[inline]
    fn next_back(&mut self) -> Option<Self::Item> {
        match self {
            Self::Required(values) => values.next_back().map(Some),
            Self::Optional(zipped) => zipped.next_back(),
        }
    }
}

impl<T, I, V> ExactSizeIterator for ZipValidity<T, I, V>
where
    I: ExactSizeIterator<Item = T>,
    V: ExactSizeIterator<Item = bool>,
{
}

impl<T, I, V> ZipValidity<T, I, V>
where
    I: Iterator<Item = T>,
    V: Iterator<Item = bool>,
{
    /// Unwrap into an iterator that has no null values.
    pub fn unwrap_required(self) -> I {
        if let Self::Required(i) = self {
            i
        } else {
            panic!("Could not 'unwrap_required'. 'ZipValidity' iterator has nulls.");
        }
    }

    /// Unwrap into an iterator that has null values.
    pub fn unwrap_optional(self) -> ZipValidityIter<T, I, V> {
        if let Self::Optional(i) = self {
            i
        } else {
            panic!("Could not 'unwrap_optional'. 'ZipValidity' iterator has no nulls.");
        }
    }
}
