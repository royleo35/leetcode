pub struct Solution;
impl Solution {
    pub fn common_chars(words: Vec<String>) -> Vec<String> {
        let mut table = vec![vec![0;26];words.len()];
        for (i,w) in words.iter().enumerate(){
            for c in w.chars() {
                table[i][(c as usize)-('a' as usize)] += 1;
            }
        }
        let mut res = Vec::new();
        for c in 0..26 {
            let mut cnt = i32::MAX;
            for r in 0..words.len() {
                cnt = cnt.min(table[r][c]);
            }
            for _ in 0..cnt {
                res.push((((c as u8)+('a' as u8)) as char).to_string());
            }
        }
        res
    }
}