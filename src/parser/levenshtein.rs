/// Compute the Levenshtein edit distance between two strings.
pub fn distance(a: &str, b: &str) -> usize {
    let a_len = a.chars().count();
    let b_len = b.chars().count();

    if a_len == 0 {
        return b_len;
    }
    if b_len == 0 {
        return a_len;
    }

    let mut prev: Vec<usize> = (0..=b_len).collect();
    let mut curr = vec![0; b_len + 1];

    for (i, ca) in a.chars().enumerate() {
        curr[0] = i + 1;
        for (j, cb) in b.chars().enumerate() {
            let cost = if ca == cb { 0 } else { 1 };
            curr[j + 1] = (prev[j] + cost).min(prev[j + 1] + 1).min(curr[j] + 1);
        }
        std::mem::swap(&mut prev, &mut curr);
    }

    prev[b_len]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_identical() {
        assert_eq!(distance("curl", "curl"), 0);
    }

    #[test]
    fn test_one_edit() {
        assert_eq!(distance("crul", "curl"), 2);
        assert_eq!(distance("cur", "curl"), 1);
    }

    #[test]
    fn test_empty() {
        assert_eq!(distance("", "abc"), 3);
        assert_eq!(distance("abc", ""), 3);
    }
}
