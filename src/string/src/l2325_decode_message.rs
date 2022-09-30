pub struct Solution;
impl Solution {
    pub fn decode_message(key: String, message: String) -> String {
        use std::collections::HashMap;
        let mut m = HashMap::new();
        m.insert(' ' as u8, ' ' as u8);
        let mut st = 'a' as u8;
        for b in key.as_bytes() {
            if m.get(&b).is_none() {
                m.insert(*b,st);
                st += 1;
            }
        }
        let mut res = String::with_capacity(message.len());
        for b in message.as_bytes() {
            res.push(*m.get(b).unwrap() as char);
        }
        res
    }
}