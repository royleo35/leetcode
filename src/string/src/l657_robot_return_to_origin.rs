pub struct Solution;
impl Solution {
    pub fn judge_circle(moves: String) -> bool {
        let mut up_down = 0;
        let mut left_right = 0;
        for c in moves.chars() {
            if c == 'U' {
                up_down += 1;
            } else if c == 'D' {
                up_down -= 1;
            } else if c == 'L' {
                left_right += 1;
            } else if c == 'R' {
                left_right -= 1;
            }
        }
        up_down == 0 && left_right == 0
    }
}