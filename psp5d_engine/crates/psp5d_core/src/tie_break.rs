use std::cmp::Ordering;

use crate::digest::digest_sha256_jcs;
use crate::errors::CoreError;
use crate::rd::RunDescriptor;
use crate::types::ScoredValue;

pub fn sort_by_score_then_digest(
    values: &mut [ScoredValue],
    rd: &RunDescriptor,
) -> Result<(), CoreError> {
    let mut decorated = Vec::with_capacity(values.len());
    for value in values.iter() {
        decorated.push((digest_sha256_jcs(&value.payload, rd)?, value.clone()));
    }

    decorated.sort_by(|(d_a, a), (d_b, b)| {
        b.score
            .partial_cmp(&a.score)
            .unwrap_or(Ordering::Equal)
            .then_with(|| d_a.cmp(d_b))
    });

    for (index, (_, value)) in decorated.into_iter().enumerate() {
        values[index] = value;
    }
    Ok(())
}
