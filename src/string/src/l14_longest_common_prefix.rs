pub struct Solution;
impl Solution {
    pub fn longest_common_prefix(strs: Vec<String>) -> String {

        // solution 1
        // let mut res = strs[0].to_string();
        // for ss in strs {
        //     res = Self::aux(res, ss);
        // }
        // res

        // solution 2
        // match strs.is_empty() {
        //     true => "".to_string(),
        //     _ => {
        //         strs.iter().skip(1).fold(strs[0].clone(), |acc, x| {
        //             acc
        //                 .chars()
        //                 .zip(x.chars())
        //                 .take_while(|(x,y)| x == y)
        //                 .map(|(x, _)| x)
        //                 .collect()
        //         })
        //     }
        // }

        // solution 3
        let mut tmp = strs[0].as_bytes();
        for i in 1..strs.len() {
            tmp = Self::prefix(tmp, strs[i].as_bytes());
        }
        tmp.iter().map(|x| *x as char).collect()
    }

    // fn aux(str1: String, str2: String) -> String {
    //     let bs1 = str1.as_bytes();
    //     let bs2 = str2.as_bytes();
    //     let mut i = 0;
    //     while i < bs1.len() && i < bs2.len() && bs1[i] == bs2[i] {
    //         i += 1;
    //     }
    //     return str1[0..i].to_string();
    // }

    fn prefix<'a>(s1: &'a [u8], s2: &'a [u8]) -> &'a [u8] {
        let l = s1.len().min(s2.len());
        for i in 0..l {
            if s1[i] != s2[i]{
                return &s1[..i];
            }
        }
        &s1[..l]
    }
}