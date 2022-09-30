pub struct Solution;
impl Solution {
    pub fn min_deletion_size(strs: Vec<String>) -> i32 {
        let col = strs[0].len();
        let row = strs.len();
        let mut cnt = 0;
        for c in 0..col {
            //let first = Self::getmn(&strs, r, c);
            for r in 1..row {
                if Self::getmn(&strs, r, c) < Self::getmn(&strs, r-1, c) {
                    cnt += 1;
                    break;
                }
            }
        }
        cnt
    }
    fn getmn(strs:&Vec<String>, m:usize, n:usize) ->char {
        strs[m].as_bytes()[n] as char
    }
}