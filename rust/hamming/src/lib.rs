/// Return the Hamming distance between the strings,
/// or None if the lengths are mismatched.
pub fn hamming_distance(s1: &str, s2: &str) -> Option<usize> {
    if s1.len() != s2.len() {
        return None;
    }
    let mut dist = 0usize;
    let mut idx = 0usize;
    while idx < s1.len() {
        if s1.chars().nth(idx) != s2.chars().nth(idx) {
            dist += 1;
        }
        idx += 1;
    }
    Some(dist)
}
