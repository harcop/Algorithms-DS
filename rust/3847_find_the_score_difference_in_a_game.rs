/// LeetCode #3847 - Find the Score Difference in a Game
fn score_difference(nums: Vec<i32>) -> i32 {
    let mut ans = 0;
    let mut k = 1;
    for (i, x) in nums.into_iter().enumerate() {
        if x % 2 != 0 {
            k = -k;
        }
        if i % 6 == 5 {
            k = -k;
        }
        ans += k * x;
    }
    ans
}

fn main() {
    println!("{}", score_difference(vec![1, 2, 3]));
}

#[cfg(test)]
mod tests {
    use super::score_difference;

    #[test]
    fn example1() {
        assert_eq!(score_difference(vec![1, 2, 3]), 0);
    }

    #[test]
    fn example2() {
        assert_eq!(score_difference(vec![2, 4, 2, 1, 2, 1]), 4);
    }

    #[test]
    fn example3() {
        assert_eq!(score_difference(vec![1]), -1);
    }
}
