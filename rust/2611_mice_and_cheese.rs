/// LeetCode #2611 - Mice and Cheese
fn mice_and_cheese(mut reward1: Vec<i32>, reward2: Vec<i32>, k: i32) -> i32 {
    let n = reward1.len();
    let mut ans = 0;
    for i in 0..n {
        ans += reward2[i];
        reward1[i] -= reward2[i];
    }
    reward1.sort_unstable_by(|a, b| b.cmp(a));
    for i in 0..k as usize {
        ans += reward1[i];
    }
    ans
}

fn main() {
    println!(
        "{}",
        mice_and_cheese(vec![1, 1, 3, 4], vec![4, 4, 1, 1], 2)
    );
}

#[cfg(test)]
mod tests {
    use super::mice_and_cheese;

    #[test]
    fn example_one() {
        assert_eq!(mice_and_cheese(vec![1, 1, 3, 4], vec![4, 4, 1, 1], 2), 15);
    }

    #[test]
    fn example_two() {
        assert_eq!(mice_and_cheese(vec![1, 1], vec![1, 1], 2), 2);
    }
}
