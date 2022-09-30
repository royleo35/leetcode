pub struct Solution;
impl Solution {
    pub fn reverse_only_letters(s: String) -> String {
        let mut bs = s.into_bytes();
        let mut l = 0;
        let mut r = bs.len()-1;
        while l < r {
            if !(bs[l] as char).is_alphabetic() {
                l += 1;
            } else if  !(bs[r] as char).is_alphabetic() {
                r -= 1;
            } else {
                bs.swap(l, r);
                l += 1;
                r -= 1;
            }
        }
        String::from_utf8(bs).unwrap()
    }
}