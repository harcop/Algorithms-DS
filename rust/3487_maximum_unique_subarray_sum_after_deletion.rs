/// LeetCode #3487 - Maximum Unique Subarray Sum After Deletion
fn max_sum(nums: Vec<i32>) -> i32 {
    let mx = *nums.iter().max().unwrap();
    if mx <= 0 {
        return mx;
    }
    let mut ans = 0;
    let mut seen = std::collections::HashSet::new();
    for x in nums {
        if x < 0 || !seen.insert(x) {
            continue;
        }
        ans += x;
    }
    ans
}

fn main() {
    println!("{}", max_sum(vec![1, 2, 3, 4, 5]));
}

#[cfg(test)]
mod tests {
    use super::max_sum;

    #[test]
    fn example1() {
        assert_eq!(max_sum(vec![1, 2, 3, 4, 5]), 15);
    }

    #[test]
    fn example2() {
        assert_eq!(max_sum(vec![1, 1, 0, 1, 1]), 1);
    }

    #[test]
    fn example3() {
        assert_eq!(max_sum(vec![1, 2, -1, -2, 1, 0, -1]), 3);
    }
}
