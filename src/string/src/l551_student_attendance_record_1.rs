pub struct Solution;
impl Solution {
    pub fn check_record(s: String) -> bool {
        // solution 1
        // let a:u32 = s.chars().collect::<Vec<char>>().into_iter().map(|x| if x== 'A'{1} else {0}).sum();
        // let b:u32 = s.chars().collect::<Vec<char>>().windows(3).map(
        //     |x| if x.len() < 3 {0} else {if x[0]=='L' && x[1] == 'L' && x[2] == 'L'{1} else {0}}
        // ).sum();
        // a < 2 && b == 0

        // solution 2
        let a = s.chars().filter(|x| *x == 'A').count();
        let b = s
            .chars()
            .collect::<Vec<char>>()
            .windows(3)
            .filter(|x| x.len() == 3 && x[0] =='L' && x[1] == 'L' && x[2] == 'L').count();
        a < 2 && b == 0

    }
}