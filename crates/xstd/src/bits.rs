//! Utilities for bit and byte manipulation

/// Increases `p` as little as possible (including possibly 0)
/// such that it becomes a multiple of `N`.
#[must_use]
pub const fn align_up<const N: usize>(p: usize) -> usize {
    if p % N == 0 {
        p
    } else {
        p + (N - (p % N))
    }
}
