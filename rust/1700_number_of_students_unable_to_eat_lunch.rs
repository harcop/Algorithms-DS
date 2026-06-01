/// LeetCode #1700 - Number Of Students Unable To Eat Lunch
use std::collections::VecDeque;

fn count_students(students: Vec<i32>, sandwiches: Vec<i32>) -> i32 {
    let mut cnt = [0i32; 2];
    for s in students { cnt[s as usize] += 1; }
    for &sw in &sandwiches {
        if cnt[sw as usize] == 0 { return cnt[0] + cnt[1]; }
        cnt[sw as usize] -= 1;
    }
    0
}
fn main() { println!("{}", count_students(vec![1,1,0,0], vec![0,1,0,1])); }
#[cfg(test)]
mod tests {
    use super::count_students;
    #[test]
    fn example_one() { assert_eq!(count_students(vec![1,1,0,0], vec![0,1,0,1]), 0); }
}