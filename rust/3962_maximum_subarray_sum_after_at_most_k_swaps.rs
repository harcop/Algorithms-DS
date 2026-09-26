/// LeetCode #3962 - Maximum Subarray Sum After at Most K Swaps
fn max_sum(nums: Vec<i32>, k: i32) -> i64 {
    let n = nums.len();
    let k = k as usize;
    let mut ans = i64::MIN;
    for l in 0..n {
        for r in l..n {
            let mut inside: Vec<i64> = nums[l..=r].iter().map(|&x| x as i64).collect();
            let mut outside: Vec<i64> = nums[..l]
                .iter()
                .chain(nums[r + 1..].iter())
                .map(|&x| x as i64)
                .collect();
            inside.sort_unstable();
            outside.sort_unstable_by(|a, b| b.cmp(a));
            let mut cur: i64 = inside.iter().sum();
            ans = ans.max(cur);
            let lim = k.min(inside.len()).min(outside.len());
            for t in 0..lim {
                if inside[t] >= outside[t] {
                    break;
                }
                cur += outside[t] - inside[t];
                ans = ans.max(cur);
            }
        }
    }
    ans
}

fn main() {
    println!("{}", max_sum(vec![1, -1, 0, 2], 1));
}

#[cfg(test)]
mod tests {
    use super::max_sum;

    #[test]
    fn example1() {
        assert_eq!(max_sum(vec![1, -1, 0, 2], 1), 3);
    }

    #[test]
    fn example2() {
        assert_eq!(max_sum(vec![4, 3, 2, 4], 2), 13);
    }

    #[test]
    fn example3() {
        assert_eq!(max_sum(vec![-1, -2], 0), -1);
    }
}
