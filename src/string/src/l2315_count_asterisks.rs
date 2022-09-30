pub struct Solution;
impl Solution {
    pub fn count_asterisks(s: String) -> i32 {
        let bs = s.as_bytes();
        let mut cnt = 0;
        let mut i = 0;
        while i < bs.len() {
            let c = bs[i] as char;
            if c == '*' {
                cnt += 1;
            } else if c == '|' { // skip '|' pair
                i += 1;
                while i<bs.len() && (bs[i] as char) != '|' {i += 1};
            }
            i += 1;
        }
        cnt
    }
}