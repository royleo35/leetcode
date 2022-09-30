pub struct Solution;
impl Solution {
    pub fn minimum_recolors(blocks: String, k: i32) -> i32 {
        blocks.chars().collect::<Vec<char>>().windows(k as usize).map(|x| x.iter().filter(|&c| *c == 'W').count()).min().unwrap_or(0) as i32
    }
}