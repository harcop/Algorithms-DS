/// LeetCode #3736 - Minimum Moves to Equal Array Elements III
fn min_moves(nums: Vec<i32>) -> i32 {
    let mx = *nums.iter().max().unwrap();
    mx * nums.len() as i32 - nums.iter().sum::<i32>()
}

fn main() {
    println!("{}", min_moves(vec![2, 1, 3]));
}

#[cfg(test)]
mod tests {
    use super::min_moves;

    #[test]
    fn example1() {
        assert_eq!(min_moves(vec![2, 1, 3]), 3);
    }

    #[test]
    fn example2() {
        assert_eq!(min_moves(vec![4, 4, 5]), 2);
    }
}
