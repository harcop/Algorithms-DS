/// LeetCode #2028 - Find Missing Observations
fn missing_rolls(rolls: Vec<i32>, mean: i32, n: i32) -> Vec<i32> {
    let m = rolls.len() as i32;
    let s = (n + m) * mean - rolls.iter().sum::<i32>();
    if s > n * 6 || s < n {
        return Vec::new();
    }
    let base = s / n;
    let rem = s % n;
    let mut ans = vec![base; n as usize];
    for i in 0..rem as usize {
        ans[i] += 1;
    }
    ans
}

fn main() {
    println!("{:?}", missing_rolls(vec![3, 2, 4, 3], 4, 2));
}

#[cfg(test)]
mod tests {
    use super::missing_rolls;

    fn valid(rolls: &[i32], mean: i32, n: i32, ans: &[i32]) -> bool {
        if ans.len() as i32 != n {
            return false;
        }
        if ans.iter().any(|&x| !(1..=6).contains(&x)) {
            return false;
        }
        let m = rolls.len() as i32;
        let total: i32 = rolls.iter().sum::<i32>() + ans.iter().sum::<i32>();
        total == (n + m) * mean
    }

    #[test]
    fn example_one() {
        let ans = missing_rolls(vec![3, 2, 4, 3], 4, 2);
        assert!(valid(&[3, 2, 4, 3], 4, 2, &ans));
    }

    #[test]
    fn example_two() {
        let ans = missing_rolls(vec![1, 5, 6], 3, 4);
        assert!(valid(&[1, 5, 6], 3, 4, &ans));
    }

    #[test]
    fn example_three() {
        assert_eq!(missing_rolls(vec![1, 2, 3, 4], 6, 4), Vec::<i32>::new());
    }
}
