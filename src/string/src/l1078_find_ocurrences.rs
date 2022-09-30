pub struct Solution;
impl Solution {
    pub fn find_ocurrences(text: String, first: String, second: String) -> Vec<String> {
        let mut res = Vec::new();
        let ss:Vec<&str> = text.split(' ').collect();
        for i in 0..ss.len()-2{
            if ss[i]==first.as_str() && ss[i+1] == second.as_str() {
                res.push(ss[i+2].to_string());
            }
        }
        res
    }
}