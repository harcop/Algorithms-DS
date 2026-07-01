/// LeetCode #2195 - Append K Integers With Minimal Sum
fn minimal_k_sum(nums: Vec<i32>, k: i32) -> i64 {
    let mut nums = nums;
    nums.push(0);
    nums.push(2_000_000_000);
    nums.sort_unstable();

    let mut ans = 0i64;
    let mut k = k as i64;
    for w in nums.windows(2) {
        let a = w[0] as i64;
        let b = w[1] as i64;
        let m = (b - a - 1).max(0).min(k);
        ans += (a + 1 + a + m) * m / 2;
        k -= m;
    }
    ans
}

fn main() {
    println!("{}", minimal_k_sum(vec![1, 4, 25, 10, 25], 2));
}

#[cfg(test)]
mod tests {
    use super::minimal_k_sum;

    #[test]
    fn example_one() {
        assert_eq!(minimal_k_sum(vec![1, 4, 25, 10, 25], 2), 5);
    }

    #[test]
    fn example_two() {
        assert_eq!(minimal_k_sum(vec![5, 6], 6), 25);
    }
}
