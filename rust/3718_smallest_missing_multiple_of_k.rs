/// LeetCode #3718 - Smallest Missing Multiple of K
use std::collections::HashSet;

fn missing_multiple(nums: Vec<i32>, k: i32) -> i32 {
    let lookup: HashSet<i32> = nums.into_iter().collect();
    let mut i = 1;
    loop {
        let v = i * k;
        if !lookup.contains(&v) {
            return v;
        }
        i += 1;
    }
}

fn main() {
    println!("{}", missing_multiple(vec![8, 2, 3, 4, 6], 2));
}

#[cfg(test)]
mod tests {
    use super::missing_multiple;

    #[test]
    fn example1() {
        assert_eq!(missing_multiple(vec![8, 2, 3, 4, 6], 2), 10);
    }

    #[test]
    fn example2() {
        assert_eq!(missing_multiple(vec![1, 4, 7, 10, 15], 5), 5);
    }
}
