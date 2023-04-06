// Copyright (c) 2023 Tony Barbitta
// 
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

//! Iterator utilities.

use std::iter::{self, Chain, Once};

/// Extension methods for iterators.
pub trait IteratorExt
where
    Self: Iterator + Sized,
{
    /// Chains a single `item` onto the end of this iterator.
    ///
    /// Equivalent to `self.chain(iter::once(item))`.
    fn chain_one(self, item: Self::Item) -> Chain<Self, Once<Self::Item>> {
        self.chain(iter::once(item))
    }

    /// Reports whether all the elements of the iterator are the same.
    ///
    /// This condition is trivially true for iterators with zero or one elements.
    fn all_equal(mut self) -> bool
    where
        Self::Item: PartialEq,
    {
        match self.next() {
            None => true,
            Some(v1) => self.all(|v2| v1 == v2),
        }
    }
}

impl<I> IteratorExt for I where I: Iterator {}

#[cfg(test)]
mod tests {
    use crate::iter::IteratorExt;

    #[test]
    fn test_all_equal() {
        let empty: [i64; 0] = [];
        assert!(empty.iter().all_equal());
        assert!([1].iter().all_equal());
        assert!([1, 1].iter().all_equal());
        assert!(![1, 2].iter().all_equal());
    }
}