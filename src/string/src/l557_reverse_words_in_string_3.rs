pub struct Solution;
impl Solution {
    pub fn reverse_words(s: String) -> String {
        let ss:Vec<String> = s.split(' ').map(|x|x.chars().rev().collect()).collect();
        ss.join(" ")
    }
}