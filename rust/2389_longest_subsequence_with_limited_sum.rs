/// LeetCode #2389 - Longest Subsequence With Limited Sum
fn answer_queries(mut nums: Vec<i32>, queries: Vec<i32>) -> Vec<i32> {
    nums.sort_unstable();
    for i in 1..nums.len() {
        nums[i] += nums[i - 1];
    }
    queries
        .into_iter()
        .map(|q| match nums.binary_search(&q) {
            Ok(idx) => idx as i32 + 1,
            Err(idx) => idx as i32,
        })
        .collect()
}

fn main() {
    println!("{:?}", answer_queries(vec![4, 5, 2, 1], vec![3, 10, 21]));
}

#[cfg(test)]
mod tests {
    use super::answer_queries;

    #[test]
    fn example_one() {
        assert_eq!(
            answer_queries(vec![4, 5, 2, 1], vec![3, 10, 21]),
            vec![2, 3, 4]
        );
    }

    #[test]
    fn example_two() {
        assert_eq!(answer_queries(vec![2, 3, 4, 5], vec![1]), vec![0]);
    }
}
