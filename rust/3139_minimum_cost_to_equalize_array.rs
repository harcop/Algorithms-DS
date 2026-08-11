/// LeetCode #3139 - Minimum Cost to Equalize Array
fn min_cost_to_equalize_array(nums: Vec<i32>, cost1: i32, cost2: i32) -> i32 {
    const MOD: i64 = 1_000_000_007;
    let n = nums.len() as i64;
    let min_num = *nums.iter().min().unwrap() as i64;
    let max_num = *nums.iter().max().unwrap() as i64;
    let sum: i64 = nums.iter().map(|&x| x as i64).sum();
    let cost1 = cost1 as i64;
    let cost2 = cost2 as i64;

    if cost1 * 2 <= cost2 || n < 3 {
        let total_gap = max_num * n - sum;
        return ((cost1 * total_gap) % MOD) as i32;
    }

    let mut ans = i64::MAX;
    let mut target = max_num;
    while target < 2 * max_num {
        let max_gap = target - min_num;
        let total_gap = target * n - sum;
        let pairs = (total_gap / 2).min(total_gap - max_gap);
        ans = ans.min(cost1 * (total_gap - 2 * pairs) + cost2 * pairs);
        target += 1;
    }
    (ans % MOD) as i32
}

fn main() {
    println!("{}", min_cost_to_equalize_array(vec![4, 1], 5, 2));
}

#[cfg(test)]
mod tests {
    use super::min_cost_to_equalize_array;

    #[test]
    fn example1() {
        assert_eq!(min_cost_to_equalize_array(vec![4, 1], 5, 2), 15);
    }

    #[test]
    fn example2() {
        assert_eq!(min_cost_to_equalize_array(vec![2, 3, 3, 3, 5], 2, 1), 6);
    }

    #[test]
    fn example3() {
        assert_eq!(min_cost_to_equalize_array(vec![3, 5, 3], 1, 3), 4);
    }
}
