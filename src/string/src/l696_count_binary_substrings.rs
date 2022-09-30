pub struct Solution;

impl Solution {
    pub fn count_binary_substrings(s: String) -> i32 {
        let mut res = 0;
        let bs = s.as_bytes();
        //  pre means prev length, curr means curr length
        let (mut pre, mut curr) = (0, 1);
        for i in 1..bs.len() {
            if bs[i] == bs[i-1] {
                curr += 1;
            } else { // swith
                pre = curr;
                curr = 1;
            }
            if pre >= curr { // if length of pre is <= length of curr, we find a substring
                res += 1;
            }
        }
        res
    }
}