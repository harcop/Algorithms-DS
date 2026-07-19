/// LeetCode #2498 - Frog Jump II
fn max_jump(stones: Vec<i32>) -> i32 {
    let mut ans = stones[1] - stones[0];
    for i in 2..stones.len() {
        ans = ans.max(stones[i] - stones[i - 2]);
    }
    ans
}

fn main() {
    println!("{}", max_jump(vec![0, 2, 5, 6, 7]));
}

#[cfg(test)]
mod tests {
    use super::max_jump;

    #[test]
    fn example_one() {
        assert_eq!(max_jump(vec![0, 2, 5, 6, 7]), 5);
    }

    #[test]
    fn example_two() {
        assert_eq!(max_jump(vec![0, 3, 9]), 9);
    }
}
