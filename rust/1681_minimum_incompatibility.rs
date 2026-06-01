/// LeetCode #1681 - Minimum Incompatibility
fn minimum_incompatibility(nums: Vec<i32>, k: i32) -> i32 {
    let n = nums.len();
    let k = k as usize;
    let sz = n / k;
    let mut dp = vec![vec![i32::MAX; 1 << n]; k + 1];
    dp[0][0] = 0;
    for mask in 0usize..(1 << n) {
        if mask.count_ones() as usize % sz != 0 {
            continue;
        }
        let mut inc = 0i32;
        let mut seen = [false; 21];
        let mut mn = 21i32;
        let mut mx = 0i32;
        for i in 0..n {
            if (mask >> i) & 1 == 1 {
                let v = nums[i];
                if seen[v as usize] {
                    inc = i32::MAX;
                    break;
                }
                seen[v as usize] = true;
                mn = mn.min(v);
                mx = mx.max(v);
            }
        }
        if inc == i32::MAX {
            continue;
        }
        inc = mx - mn;
        for prev in 0usize..mask {
            if prev | mask != mask {
                continue;
            }
            if prev.count_ones() as usize % sz != 0 {
                continue;
            }
            let parts = mask.count_ones() as usize / sz;
            if parts <= k && dp[parts - 1][prev] != i32::MAX {
                dp[parts][mask] = dp[parts][mask].min(dp[parts - 1][prev] + inc);
            }
        }
    }
    let ans = dp[k][(1 << n) - 1];
    if ans == i32::MAX { -1 } else { ans }
}

fn main() {
    println!("{}", minimum_incompatibility(vec![1, 2, 1, 4], 2));
}

#[cfg(test)]
mod tests {
    use super::minimum_incompatibility;

    #[test]
    fn example_one() {
        assert_eq!(minimum_incompatibility(vec![1, 2, 1, 4], 2), 1);
    }
}
