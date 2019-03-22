//! Functions related to searching through a set of strings with queries
use std::f64::NAN;

use rff::match_and_score;

use crate::query::QueryData;

/// Function that maps the score from the interval (-inf, inf) into the interval (0, 1)
fn score_map(x: f64) -> f64 {
    1.0 / (1.0 + (-x).exp())
}

/// Gives a `QueryData` a rank, searching with a `&Vec<String>`
pub fn rank_query(query: &QueryData, search_strings: &Vec<&str>) -> f64 {
    search_strings
        .iter()
        .zip(&query.strings)
        .map(
            |(needle, haystack)| match match_and_score(needle, haystack) {
                Some((_, score)) => score_map(score),
                None => NAN,
            },
        )
        .sum::<f64>()
}

/// Gives a `QueryData` a rank, searching with a `&Vec<String>` and weighting the different searches with a `&Vec<f64>`
pub fn rank_query_weighted(
    query: &QueryData,
    search_strings: &Vec<&str>,
    weights: &Vec<f64>,
) -> f64 {
    search_strings
        .iter()
        .zip(&query.strings)
        .zip(weights)
        .map(
            |((needle, haystack), weight)| match match_and_score(needle, haystack) {
                Some((_, score)) => score_map(score) * weight,
                None => NAN,
            },
        )
        .sum::<f64>()
}
