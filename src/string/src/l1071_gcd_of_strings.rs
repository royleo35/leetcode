
pub struct Solution;
impl Solution {
    pub fn gcd_of_strings(str1: String, str2: String) -> String {
        match str1.chars().chain(str2.chars()).eq(str2.chars().chain(str1.chars())) {
            true => str1.chars().take(Self::gcd(str1.len(), str2.len())).collect(),
            false => "".to_string(),
        }
    }

    fn gcd(a:usize, b:usize) -> usize {
        if b == 0 {
            return a
        }
        return Self::gcd(b, a%b)
    }
}