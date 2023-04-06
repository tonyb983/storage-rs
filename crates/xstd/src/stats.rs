//! Statistics utilities.

/// A standard range of buckets for timing data, measured in seconds.
/// Individual histograms may only need a subset of this range, in which case,
/// see `histogram_seconds_buckets` below.
///
/// Note that any changes to this range may modify buckets for existing metrics.
const HISTOGRAM_SECOND_BUCKETS: [f64; 19] = [
    0.000_128, 0.000_256, 0.000_512, 0.001, 0.002, 0.004, 0.008, 0.016, 0.032, 0.064, 0.128, 0.256,
    0.512, 1.0, 2.0, 4.0, 8.0, 16.0, 32.0,
];

/// Returns a `Vec` of time buckets that are both present in our standard
/// buckets above and within the provided inclusive range. (This makes it
/// more meaningful to compare latency percentiles across histograms if needed,
/// without requiring all metrics to use exactly the same buckets.)
#[must_use]
pub fn histogram_seconds_buckets(from: f64, to: f64) -> Vec<f64> {
    let mut vec = Vec::with_capacity(HISTOGRAM_SECOND_BUCKETS.len());
    vec.extend(
        HISTOGRAM_SECOND_BUCKETS
            .iter()
            .copied()
            .filter(|&b| b >= from && b <= to),
    );
    vec
}

/// Buckets that capture sizes of 64 bytes up to a gigabyte
pub const HISTOGRAM_BYTE_BUCKETS: [f64; 7] = [
    64.0,
    1024.0,
    16384.0,
    262_144.0,
    4_194_304.0,
    67_108_864.0,
    1_073_741_824.0,
];
