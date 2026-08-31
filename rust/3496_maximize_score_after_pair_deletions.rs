/// LeetCode #3496 - Maximize Score After Pair Deletions
fn max_score(nums: Vec<i32>) -> i32 {
    let s: i32 = nums.iter().sum();
    if nums.len() % 2 == 1 {
        s - nums.iter().copied().min().unwrap()
    } else {
        s - nums.windows(2).map(|w| w[0] + w[1]).min().unwrap()
    }
}

fn main() {
    println!("{}", max_score(vec![2, 4, 1]));
}

#[cfg(test)]
mod tests {
    use super::max_score;

    #[test]
    fn example1() {
        assert_eq!(max_score(vec![2, 4, 1]), 6);
    }

    #[test]
    fn example2() {
        assert_eq!(max_score(vec![5, -1, 4, 2]), 7);
    }
}
