pub struct Solution;
impl Solution {
    pub fn detect_capital_use(word: String) -> bool {
        let cnt = word.chars().filter(|x| x.is_uppercase()).count();
        cnt == word.len() || cnt == 0 || (cnt == 1 && (word.as_bytes()[0] as char).is_uppercase())
    }
}