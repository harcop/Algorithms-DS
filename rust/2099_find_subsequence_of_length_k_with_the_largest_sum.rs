/// LeetCode #2099 - Find Subsequence of Length K With the Largest Sum
fn max_subsequence(nums: Vec<i32>, k: i32) -> Vec<i32> {
    let mut idx: Vec<usize> = (0..nums.len()).collect();
    idx.sort_unstable_by(|&a, &b| nums[b].cmp(&nums[a]));
    idx.truncate(k as usize);
    idx.sort_unstable();
    idx.into_iter().map(|i| nums[i]).collect()
}

fn main() {
    println!("{:?}", max_subsequence(vec![2, 1, 3, 3], 2));
}

#[cfg(test)]
mod tests {
    use super::max_subsequence;

    #[test]
    fn example_one() {
        assert_eq!(max_subsequence(vec![2, 1, 3, 3], 2), vec![3, 3]);
    }

    #[test]
    fn example_two() {
        assert_eq!(max_subsequence(vec![-1, -2, 3, 4], 3), vec![-1, 3, 4]);
    }

    #[test]
    fn example_three() {
        assert_eq!(max_subsequence(vec![3, 4, 3, 3], 2), vec![3, 4]);
    }
}
