/// LeetCode #2587 - Rearrange Array to Maximize Prefix Score
fn max_score(mut nums: Vec<i32>) -> i32 {
    nums.sort_unstable_by(|a, b| b.cmp(a));
    let mut s = 0i64;
    for (i, &x) in nums.iter().enumerate() {
        s += x as i64;
        if s <= 0 {
            return i as i32;
        }
    }
    nums.len() as i32
}

fn main() {
    println!("{}", max_score(vec![2, -1, 0, 1, -3, 3, -3]));
}

#[cfg(test)]
mod tests {
    use super::max_score;

    #[test]
    fn example_one() {
        assert_eq!(max_score(vec![2, -1, 0, 1, -3, 3, -3]), 6);
    }

    #[test]
    fn example_two() {
        assert_eq!(max_score(vec![-2, -3, 0]), 0);
    }
}
