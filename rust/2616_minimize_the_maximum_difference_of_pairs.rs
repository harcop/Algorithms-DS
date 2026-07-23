/// LeetCode #2616 - Minimize the Maximum Difference of Pairs
fn minimize_max(mut nums: Vec<i32>, p: i32) -> i32 {
    nums.sort_unstable();
    let n = nums.len();
    let mut l = 0;
    let mut r = nums[n - 1] - nums[0] + 1;

    let check = |diff: i32| -> bool {
        let mut cnt = 0;
        let mut i = 0;
        while i < n - 1 {
            if nums[i + 1] - nums[i] <= diff {
                cnt += 1;
                i += 2;
            } else {
                i += 1;
            }
        }
        cnt >= p
    };

    while l < r {
        let mid = (l + r) / 2;
        if check(mid) {
            r = mid;
        } else {
            l = mid + 1;
        }
    }
    l
}

fn main() {
    println!("{}", minimize_max(vec![10, 1, 2, 7, 1, 3], 2));
}

#[cfg(test)]
mod tests {
    use super::minimize_max;

    #[test]
    fn example_one() {
        assert_eq!(minimize_max(vec![10, 1, 2, 7, 1, 3], 2), 1);
    }

    #[test]
    fn example_two() {
        assert_eq!(minimize_max(vec![4, 2, 1, 2], 1), 0);
    }
}
