pub struct Solution;
impl Solution {
    pub fn remove_outer_parentheses(s: String) -> String {
        let bs = s.as_bytes();
        let mut stack = Vec::new();
        let mut res = String::new();
        let mut st = 0;
        for (i, b) in bs.iter().enumerate() {
            if (*b as char) == '(' {
                stack.push(b);
            } else if stack.len() != 0 && (*b as char) == ')' {
                stack.pop();
            }
            if stack.len() == 0 {// got a pair
                res += &String::from_utf8(bs[st+1..i].to_vec()).unwrap().as_str();
                st = i+1;
            }
        }
        res
    }
}