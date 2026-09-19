/// LeetCode #3806 - Maximum Bitwise AND After Increment Operations
fn maximum_and(nums: Vec<i32>, k: i32, m: i32) -> i32 {
    let max_val = *nums.iter().max().unwrap() as i64 + k as i64;
    let mx = if max_val == 0 {
        0
    } else {
        64 - max_val.leading_zeros()
    };
    let n = nums.len();
    let m = m as usize;
    let k = k as i64;
    let mut ans = 0i32;
    let mut cost = vec![0i64; n];
    for bit in (0..mx).rev() {
        let target = ans | (1 << bit);
        for i in 0..n {
            let x = nums[i];
            let diff = target & !x;
            let j = if diff == 0 {
                0
            } else {
                32 - diff.leading_zeros()
            };
            let mask = (1i64 << j) - 1;
            cost[i] = (target as i64 & mask) - (x as i64 & mask);
        }
        cost.sort_unstable();
        let sum: i64 = cost.iter().take(m).sum();
        if sum <= k {
            ans = target;
        }
    }
    ans
}

fn main() {
    println!("{}", maximum_and(vec![3, 1, 2], 8, 2));
}

#[cfg(test)]
mod tests {
    use super::maximum_and;

    #[test]
    fn example1() {
        assert_eq!(maximum_and(vec![3, 1, 2], 8, 2), 6);
    }

    #[test]
    fn example2() {
        assert_eq!(maximum_and(vec![1, 2, 8, 4], 7, 3), 4);
    }

    #[test]
    fn example3() {
        assert_eq!(maximum_and(vec![1, 1], 3, 2), 2);
    }
}
