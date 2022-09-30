pub struct Solution;
impl Solution {
    pub fn rotate_string(s: String, goal: String) -> bool {
        // solution1:
        // s.len() == goal.len() && format!("{}{}",s,s).find(goal.as_str()).is_some()

        // solution2:
        s.len() == goal.len() && s.repeat(2).find(goal.as_str()).is_some()
    }
}