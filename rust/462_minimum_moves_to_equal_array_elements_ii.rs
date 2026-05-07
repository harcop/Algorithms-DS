/// LeetCode #462 - Minimum Moves to Equal Array Elements II
fn min_moves2(mut nums: Vec<i32>) -> i32 {
    nums.sort_unstable();
    let mid = nums[nums.len() / 2];
    nums.iter().map(|x| (x - mid).abs()).sum()
}

fn main() {
    println!("{}", min_moves2(vec![1, 2, 3]));
}

#[cfg(test)]
mod tests {
    use super::min_moves2;

    #[test]
    fn example_one() {
        assert_eq!(min_moves2(vec![1, 2, 3]), 2);
    }
}
