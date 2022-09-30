pub struct Solution;
impl Solution {
    pub fn num_jewels_in_stones(jewels: String, stones: String) -> i32 {
        // solution1
        // use std::collections::HashSet;
        // let s:HashSet<u8>= HashSet::from_iter(jewels.as_bytes().iter().copied());
        // stones.as_bytes().iter().map(|x| if s.contains(x) {1} else {0}).sum::<i32>()

        // solution2
        // stones.chars().filter(|&x| jewels.contains(x)).count() as i32
        //stones.chars().filter(|x| jewels.contains(*x)).count() as i32

        stones.chars().map(|x| if jewels.contains(x) {1} else {0}).sum::<i32>()

    }
}