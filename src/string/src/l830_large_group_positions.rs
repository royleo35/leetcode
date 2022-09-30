pub struct Solution;
impl Solution {
    pub fn large_group_positions(s: String) -> Vec<Vec<i32>> {
        let bs = s.as_bytes();
        let mut cnt = 1;
        let mut res = Vec::new();
        for i in 1..bs.len() {
            if bs[i] == bs[i-1] {
                cnt += 1;
            } else{// switch
                if cnt >= 3 {
                    res.push(vec![(i-cnt) as i32, (i-1) as i32]);
                }
                cnt = 1; // reset
            }
        }
        // push last one
        if cnt >= 3 {
            res.push(vec![(bs.len()-cnt) as i32, (bs.len()-1) as i32])
        }
        res
    }
}