/// LeetCode #2293 - Min Max Game
fn min_max_game(mut nums: Vec<i32>) -> i32 {
    let mut n = nums.len();
    while n > 1 {
        n >>= 1;
        for i in 0..n {
            let a = nums[i << 1];
            let b = nums[(i << 1) | 1];
            nums[i] = if i % 2 == 0 { a.min(b) } else { a.max(b) };
        }
    }
    nums[0]
}

fn main() {
    println!("{}", min_max_game(vec![1, 3, 5, 2, 4, 8, 2, 2]));
}

#[cfg(test)]
mod tests {
    use super::min_max_game;

    #[test]
    fn example_one() {
        assert_eq!(min_max_game(vec![1, 3, 5, 2, 4, 8, 2, 2]), 1);
    }

    #[test]
    fn example_two() {
        assert_eq!(min_max_game(vec![3]), 3);
    }
}
