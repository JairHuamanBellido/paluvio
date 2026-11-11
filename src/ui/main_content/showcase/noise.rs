//! Deterministic pseudo-random numbers for showcase sample data.
//!
//! Same `(seed, index)` always gives the same value, so fake data (activity
//! grids, chart lines) stays stable across renders without storing it.

/// Mixes `seed` and `index` into a well-spread `u32`.
pub fn hash(seed: u32, index: usize) -> u32 {
    #[expect(
        clippy::cast_possible_truncation,
        reason = "only the low bits matter for mixing"
    )]
    let index = index as u32;
    let mut hash = seed.wrapping_mul(2_654_435_761) ^ index.wrapping_mul(40_503);
    hash ^= hash >> 13;
    hash = hash.wrapping_mul(1_274_126_177);
    hash ^= hash >> 16;
    hash
}
