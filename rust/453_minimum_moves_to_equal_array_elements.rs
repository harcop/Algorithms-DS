/// LeetCode #453 - Minimum Moves to Equal Array Elements
fn min_moves(nums: Vec<i32>) -> i32 {
    let min = *nums.iter().min().unwrap();
    nums.iter().map(|x| x - min).sum()
}

fn main() {
    println!("{}", min_moves(vec![1, 2, 3]));
}

#[cfg(test)]
mod tests {
    use super::min_moves;

    #[test]
    fn example_one() {
        assert_eq!(min_moves(vec![1, 2, 3]), 3);
    }
}
