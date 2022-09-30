pub struct Solution;
impl Solution {
    pub fn di_string_match(s: String) -> Vec<i32> {
        let n = s.len();
        let mut res = Vec::with_capacity(n+1);
        let mut st = 0 as i32; // min
        let mut end = n as i32; // max
        for c in s.chars() {
            if c == 'I' {
                res.push(st);
                st += 1;
            } else if c == 'D' {
                res.push(end);
                end -= 1;
            }
        }
        res.push(st);
        res
    }
}