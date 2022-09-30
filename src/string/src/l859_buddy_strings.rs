pub struct Solution;
impl Solution {
    pub fn buddy_strings(s: String, goal: String) -> bool {
        let s1 = s.as_bytes();
        let s2 = goal.as_bytes();
        if s1.len() != s2.len() {
            return false;
        }
        let mut s = std::collections::HashSet::new();
        let mut not_same = Vec::new();
        for i in 0..s1.len() {
            if s1[i] != s2[i] {
                not_same.push(i);
                if not_same.len() > 2 {
                    return false;
                }
            }
            s.insert(s1[i]);
        }
        (not_same.len() == 0 && s.len() < s1.len()) || (not_same.len() == 2 && s1[not_same[0]] == s2[not_same[1]] && s1[not_same[1]] == s2[not_same[0]])
    }

}