pub struct Solution;
impl Solution {
    pub fn roman_to_int(s: String) -> i32 {
        let bs = s.as_bytes();
        let mut res = 0;
        for i in 0..bs.len() {
            match bs[i] as char {
                'M' => res += 1000,
                'D' => res += 500,
                'C' => {
                    res += 100;
                    if i < bs.len() - 1 && ((bs[i + 1] as char) == 'M' || (bs[i + 1] as char) == 'D') {
                        res -= 200;
                    }
                },
                'L' => {
                    res += 50;
                },
                'X' => {
                    res += 10;
                    if i < bs.len() - 1 && ((bs[i + 1] as char) == 'L' || (bs[i + 1] as char) == 'C') {
                        res -= 20;
                    }
                },
                'V' => res += 5,
                'I' => {
                    res += 1;
                    if i < bs.len() - 1 && ((bs[i + 1] as char) == 'V' || (bs[i + 1] as char) == 'X') {
                        res -= 2;
                    }
                },
                _ => (),
            }
        }
        res
    }
}