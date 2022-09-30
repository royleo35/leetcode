pub struct Solution;
use std::collections::HashMap;
impl Solution {
    pub fn is_alien_sorted(words: Vec<String>, order: String) -> bool {
        let mut m = std::collections::HashMap::new();
        for (i,c) in order.chars().enumerate() {
            m.insert(c as u8, i);
        }
        for i in 1..words.len() {
            if !Self::greater(&words[i], &words[i-1], &m) {
                return false;
            }
        }
        true

    }
    fn greater(s1:&String, s2: &String, m: &HashMap<u8, usize>) -> bool {
        let s1 = s1.as_bytes();
        let s2 = s2.as_bytes();
        let mut i = 0;
        while i < s1.len() && i < s2.len() {
            let v1 = m.get(&s1[i]).unwrap();
            let v2 = m.get(&s2[i]).unwrap();
            if v1 < v2 {
                return false;
            } else if v1 > v2 {
                return true;
            }
            i += 1;
        }
        s1.len() >= s2.len()
    }
}