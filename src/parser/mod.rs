#![allow(dead_code)]

pub mod levenshtein;

/// A parsed user intent ready for dispatch.
pub struct Intent {
    pub command: String,
    pub args: Vec<String>,
}

/// Find the closest matching command name using Levenshtein distance.
/// Returns `Some((name, distance))` if a match is found within `max_distance`.
pub fn fuzzy_match_command<'a>(
    input: &str,
    candidates: &[&'a str],
    max_distance: usize,
) -> Option<(&'a str, usize)> {
    candidates
        .iter()
        .map(|&c| (c, levenshtein::distance(input, c)))
        .filter(|(_, d)| *d <= max_distance)
        .min_by_key(|(_, d)| *d)
}
