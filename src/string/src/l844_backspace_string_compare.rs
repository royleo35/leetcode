pub struct Solution;
impl Solution {
    pub fn backspace_compare(s: String, t: String) -> bool {
        Self::helper(s) == Self::helper(t)
    }
    fn helper(s: String) -> String {
        let mut stack = Vec::new();
        for c in s.chars() {
            if c == '#' && stack.len() != 0{
                stack.pop();
            } else if c != '#'{
                stack.push(c);
            }
        }
        // println!("{:?}", stack);
        stack.iter().collect::<String>()
    }
}