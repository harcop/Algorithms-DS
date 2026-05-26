/// LeetCode #1403 - Minimum Subsequence In Non Increasing Order
fn min_subsequence(nums: Vec<i32>) -> Vec<i32> {
    let mut sorted = nums;
    sorted.sort_unstable_by(|a, b| b.cmp(a));
    let total: i64 = sorted.iter().map(|&x| x as i64).sum();
    let mut sum = 0i64;
    let mut ans = Vec::new();
    for x in sorted {
        sum += x as i64;
        ans.push(x);
        if sum > total - sum {
            break;
        }
    }
    ans
}

fn main() {
    println!("{:?}", min_subsequence(vec![4, 3, 10, 9, 8]));
}

#[cfg(test)]
mod tests {
    use super::min_subsequence;

    #[test]
    fn example_one() {
        assert_eq!(min_subsequence(vec![4, 3, 10, 9, 8]), vec![10, 9]);
    }

    #[test]
    fn example_two() {
        assert_eq!(min_subsequence(vec![4, 4, 7, 6, 7]), vec![7, 7, 6]);
    }
}

