pub struct Solution;
impl Solution {
    pub fn uncommon_from_sentences(s1: String, s2: String) -> Vec<String> {
        use std::collections::HashMap;
        let mut m1 = HashMap::new();
        let mut m2 = HashMap::new();
        for word in s1.split(' ') {
            *m1.entry(word).or_insert(0) += 1;
        }
        for word in s2.split(' ') {
            *m2.entry(word).or_insert(0) += 1;
        }
        let mut res = Vec::new();
        for (k,v) in &m1 {
            if *v == 1 && m2.get(k).is_none() {
                res.push(k.to_string());
            }
        }
        for (k, v) in &m2 {
            if *v == 1 && m1.get(k).is_none() {
                res.push(k.to_string());
            }
        }
        res
    }
}