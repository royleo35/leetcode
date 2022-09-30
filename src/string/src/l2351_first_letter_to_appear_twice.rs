pub struct Solution;
impl Solution {
    pub fn repeated_character(s: String) -> char {
        use std::collections::HashSet;
        let mut m:HashSet<char> = HashSet::new();
        for c in s.chars() {
            if m.contains(&c) {
                return c;
            }
            m.insert(c);
        }
        panic!("ops");
    }
}