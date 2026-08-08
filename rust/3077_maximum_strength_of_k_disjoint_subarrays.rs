/// LeetCode #3077 - Maximum Strength of K Disjoint Subarrays
fn maximum_strength(nums: Vec<i32>, k: i32) -> i64 {
    let n = nums.len();
    let k = k as usize;
    let neg = i64::MIN / 4;

    // f[j][0] = j subarrays, current not taken; f[j][1] = taken
    let mut prev = vec![[neg; 2]; k + 1];
    prev[0][0] = 0;
    prev[0][1] = 0;

    for i in 1..=n {
        let mut cur = vec![[neg; 2]; k + 1];
        cur[0][0] = 0;
        let x = nums[i - 1] as i64;
        for j in 1..=k.min(i) {
            let sign = if j % 2 == 1 { 1i64 } else { -1i64 };
            let contrib = sign * x * (k as i64 - j as i64 + 1);

            // not take nums[i-1]
            cur[j][0] = prev[j][0].max(prev[j][1]);

            // take: continue same subarray
            let cont = if prev[j][1] > neg / 2 {
                prev[j][1] + contrib
            } else {
                neg
            };
            // take: start new subarray
            let start = if prev[j - 1][0].max(prev[j - 1][1]) > neg / 2 {
                prev[j - 1][0].max(prev[j - 1][1]) + contrib
            } else {
                neg
            };
            cur[j][1] = cont.max(start);
        }
        prev = cur;
    }

    prev[k][0].max(prev[k][1])
}

fn main() {
    println!("{}", maximum_strength(vec![1, 2, 3, -1, 2], 3));
}

#[cfg(test)]
mod tests {
    use super::maximum_strength;

    #[test]
    fn example1() {
        assert_eq!(maximum_strength(vec![1, 2, 3, -1, 2], 3), 22);
    }

    #[test]
    fn example2() {
        assert_eq!(maximum_strength(vec![12, -2, -2, -2, -2], 5), 64);
    }

    #[test]
    fn example3() {
        assert_eq!(maximum_strength(vec![-1, -2, -3], 1), -1);
    }
}
