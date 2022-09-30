pub struct Solution;
impl Solution {
    pub fn check_distances(s: String, distance: Vec<i32>) -> bool {
        use std::collections::HashMap;
        let mut m:HashMap<usize,usize> = HashMap::new();
        let bs = s.as_bytes();
        for i in 0..bs.len() {
            let idx = (bs[i] as usize) - ('a' as usize);
            if let Some(first) = m.get(&idx) {
                if i - *first != (distance[idx] +1) as usize {
                    return false;
                }
            }
            m.insert(idx, i);
        }
        true
    }
}