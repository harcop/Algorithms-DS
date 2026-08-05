/// LeetCode #3013 - Divide an Array Into Subarrays With Minimum Cost II
fn minimum_cost(nums: Vec<i32>, k: i32, dist: i32) -> i64 {
    let n = nums.len();
    let k = k as usize;
    let dist = dist as usize;
    let base = nums[0] as i64;
    let mut ans = i64::MAX;

    let evaluate = |window: &[i32]| -> i64 {
        let mut v: Vec<i32> = window.to_vec();
        v.sort_unstable();
        base + v.iter().take(k - 1).map(|&x| x as i64).sum::<i64>()
    };

    for start in 1..n {
        let end = start + dist;
        if end >= n {
            break;
        }
        let window = &nums[start..=end];
        if window.len() >= k - 1 {
            ans = ans.min(evaluate(window));
        }
    }

    ans
}

fn main() {
    println!("{}", minimum_cost(vec![1, 3, 2, 6, 4, 2], 3, 3));
    println!("{}", minimum_cost(vec![10, 1, 2, 2, 2, 1], 4, 3));
    println!("{}", minimum_cost(vec![10, 8, 18, 9], 3, 1));
}

#[cfg(test)]
mod tests {
    use super::minimum_cost;

    #[test]
    fn example_one() {
        assert_eq!(minimum_cost(vec![1, 3, 2, 6, 4, 2], 3, 3), 5);
    }

    #[test]
    fn example_two() {
        assert_eq!(minimum_cost(vec![10, 1, 2, 2, 2, 1], 4, 3), 15);
    }

    #[test]
    fn example_three() {
        assert_eq!(minimum_cost(vec![10, 8, 18, 9], 3, 1), 36);
    }
}
