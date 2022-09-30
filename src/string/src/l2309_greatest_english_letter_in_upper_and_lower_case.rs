pub struct Solution;
impl Solution {

    pub fn greatest_letter(s: String) -> String {
        use std::collections::HashSet;
        let mut m = HashSet::new();
        let mut max_char = 0 as char;
        for c in s.chars() {
            let up = Self::upper(c);
            if up > max_char && m.contains(&Self::switch(c)) {
                max_char = up;
            } else {
                m.insert(c);
            }
        }
        if max_char as u8 == 0 {"".to_string()} else {max_char.to_string()}
    }
    fn switch(c:char) -> char {
        if 'a' <= c && c <= 'z' {
            ((c as u8) -('a' as u8)+('A' as u8)) as char
        } else {
            ((c as u8) + ('a' as u8)-('A' as u8)) as char
        }
    }
    fn upper(c:char) -> char {
        if 'a' <= c && c <= 'z' {
            ((c as u8) -('a' as u8)+('A' as u8)) as char
        } else {
            c
        }
    }
}