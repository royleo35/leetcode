pub struct Solution;
impl Solution {
    pub fn reverse_str(s: String, k: i32) -> String {
        s.chars().collect::<Vec<char>>().chunks_mut(2*k as usize).map(|x|{
            if x.len() > (k as usize) {
                x[0..=(k-1) as usize].reverse();
            } else {
                x.reverse();
            }
            x.to_vec()
        }).collect::<Vec<Vec<char>>>().concat().into_iter().collect::<String>()
    }
}