pub struct Solution;
impl Solution {
    pub fn find_lu_slength(a: String, b: String) -> i32 {
        // solution 1
        // if a == b {-1} else {std::cmp::max(a.len(), b.len()) as i32}

        // solution 2
        if a == b {-1} else {a.len().max(b.len())as i32}
    }
}