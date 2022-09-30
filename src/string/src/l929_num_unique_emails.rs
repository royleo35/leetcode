pub struct Solution;
impl Solution {
    pub fn num_unique_emails(emails: Vec<String>) -> i32 {
        let mut m = std::collections::HashSet::new();
        for s in emails {
            let bs = s.as_bytes();
            let mut e = String::with_capacity(bs.len());
            let mut i = 0;
            while i < bs.len() {
                let c = bs[i] as char;
                if c == '.'{
                    i += 1;
                } else if c == '+' || c == '@'{// got @ later
                    while i < bs.len() && (bs[i] as char) != '@' {
                        i += 1;
                    }
                    while i < bs.len() {
                        e.push(bs[i] as char);
                        i += 1;
                    }
                    break;
                } else {
                    e.push(c);
                    i += 1; // c == '.'
                }

            }
            m.insert(e);
        }
        m.len() as i32
    }
}