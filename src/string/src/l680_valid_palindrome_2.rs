pub struct Solution;
impl Solution {
    pub fn valid_palindrome(s: String) -> bool {
        let bs = &s.as_bytes();
        let mut l = 0;
        let mut r = bs.len()-1;
        while l < r {
            if bs[l] == bs[r] {
                l += 1;
                r -= 1;
            } else {
                return Self::is_palindrome(bs, l+1, r) || Self::is_palindrome(bs, l, r-1);
            }
        }
        true
    }
    fn is_palindrome(s: &[u8], mut l: usize,mut r: usize) -> bool {
        if l > r {
            return false;
        }
        while l < r {
            if s[l] != s[r]{
                return false;
            }
            l += 1;
            r -= 1;
        }
        true
    }
}