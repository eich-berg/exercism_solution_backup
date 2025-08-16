/// Return the Hamming distance between the strings,
/// or None if the lengths are mismatched.
pub fn hamming_distance(s1: &str, s2: &str) -> Option<usize> {
    //todo!("What is the Hamming Distance between {s1} and {s2}");
    if s1.len() == s2.len() {
        return Some(s1.chars().zip(s2.chars()).filter(|(a, b)| a != b).count());
    }
    None
}
