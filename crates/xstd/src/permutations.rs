//! Functions for working with permutations

use std::collections::BTreeMap;

/// Given a permutation, construct its inverse.
pub fn invert<I>(permutation: I) -> impl Iterator<Item = (usize, usize)>
where
    I: IntoIterator<Item = usize>,
{
    permutation.into_iter().enumerate().map(|(idx, c)| (c, idx))
}

/// Construct the permutation that sorts `data`.
pub fn argsort<T>(data: &[T]) -> Vec<usize>
where
    T: Ord,
{
    let mut indices = (0..data.len()).collect::<Vec<_>>();
    indices.sort_by_key(|&i| &data[i]);
    indices
}

/// Construct the permutation that takes `data.sorted()` to `data`.
pub fn inverse_argsort<T>(data: &[T]) -> Vec<usize>
where
    T: Ord,
{
    let map = invert(argsort(data)).collect::<BTreeMap<_, _>>();
    (0..data.len()).map(|i| map[&i]).collect()
}
