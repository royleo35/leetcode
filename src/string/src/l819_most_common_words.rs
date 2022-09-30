pub struct Solution;
impl Solution {
    pub fn most_common_word(paragraph: String, banned: Vec<String>) -> String {
        let ss = Self::split(&paragraph);
        let mut m = std::collections::HashMap::new();
        let mut m2 = std::collections::HashSet::new();
        for b in banned {
            m2.insert(b.to_ascii_lowercase());
        }
        let mut idx = 0;
        let mut max = 0;
        for (i,s) in ss.iter().enumerate() {
            if !m2.contains(s) {
                let cnt = m.entry(s).or_insert(0);
                *cnt += 1;
                if *cnt > max {
                    max = *cnt;
                    idx = i;
                }
            }

        }
        ss[idx].clone()
    }
    fn split(s: &String) -> Vec<String> {
        let bs = s.as_bytes();
        let mut i = 0;
        let mut res = Vec::new();
        while i < bs.len() {
            // find word
            // skip not alpha
            while i < bs.len() && !(bs[i] as char).is_alphabetic(){i +=1;}
            let st = i;
            // got a word
            while i < bs.len() && (bs[i] as char).is_alphabetic(){i +=1;}
            if i > st {
                res.push(bs[st..i].iter().map(|x| {
                    (*x as char).to_lowercase().to_string()
                }).collect::<String>());
            }
        }
        res
    }
}