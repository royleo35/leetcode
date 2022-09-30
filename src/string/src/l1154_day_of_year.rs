pub struct Solution;
impl Solution {
    pub fn day_of_year(date: String) -> i32 {
        let ss:Vec<&str> = date.split('-').collect();
        let (year, month, day) = (str::parse::<i32>(ss[0]).unwrap(), str::parse::<i32>(ss[1]).unwrap(), str::parse::<i32>(ss[2]).unwrap());
        let mut days = [31,28,31,30,31,30,31,31,30,31,30,31];
        if (year % 100 == 0 && year % 400 == 0) || (year % 100 != 0 && year % 4 == 0) {
            days[1] = 29;
        }
        let mut res = 0;
        for i in 0..month-1 {
            res += days[i as usize];
        }
        res += day;
        res
    }
}