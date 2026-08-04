/// LeetCode #2974 - Minimum Number Game
fn number_game(mut nums: Vec<i32>) -> Vec<i32> {
    nums.sort_unstable();
    let mut ans = Vec::with_capacity(nums.len());
    for chunk in nums.chunks(2) {
        ans.push(chunk[1]);
        ans.push(chunk[0]);
    }
    ans
}

fn main() {
    println!("{:?}", number_game(vec![5, 4, 2, 3]));
}

#[cfg(test)]
mod tests {
    use super::number_game;

    #[test]
    fn example_one() {
        assert_eq!(number_game(vec![5, 4, 2, 3]), vec![3, 2, 5, 4]);
    }

    #[test]
    fn example_two() {
        assert_eq!(number_game(vec![2, 5]), vec![5, 2]);
    }
}
