/// LeetCode #3489 - Zero Array Transformation IV
fn can_make(nums: &[i32], queries: &[Vec<i32>], k: usize) -> bool {
    for (i, &target) in nums.iter().enumerate() {
        if target == 0 {
            continue;
        }
        let t = target as usize;
        let mut dp = vec![false; t + 1];
        dp[0] = true;
        for q in queries.iter().take(k) {
            if q[0] as usize <= i && i <= q[1] as usize {
                let v = q[2] as usize;
                for j in (v..=t).rev() {
                    if dp[j - v] {
                        dp[j] = true;
                    }
                }
            }
        }
        if !dp[t] {
            return false;
        }
    }
    true
}

fn min_zero_array(nums: Vec<i32>, queries: Vec<Vec<i32>>) -> i32 {
    if nums.iter().all(|&x| x == 0) {
        return 0;
    }
    let mut lo = 0;
    let mut hi = queries.len() as i32;
    let mut ans = -1;
    while lo <= hi {
        let mid = (lo + hi) / 2;
        if can_make(&nums, &queries, mid as usize) {
            ans = mid;
            hi = mid - 1;
        } else {
            lo = mid + 1;
        }
    }
    ans
}

fn main() {
    println!(
        "{}",
        min_zero_array(vec![2, 0, 2], vec![vec![0, 2, 1], vec![0, 2, 1], vec![1, 1, 3]])
    );
}

#[cfg(test)]
mod tests {
    use super::min_zero_array;

    #[test]
    fn example1() {
        assert_eq!(
            min_zero_array(vec![2, 0, 2], vec![vec![0, 2, 1], vec![0, 2, 1], vec![1, 1, 3]]),
            2
        );
    }

    #[test]
    fn example2() {
        assert_eq!(
            min_zero_array(vec![4, 3, 2, 1], vec![vec![1, 3, 2], vec![0, 2, 1]]),
            -1
        );
    }

    #[test]
    fn example3() {
        assert_eq!(
            min_zero_array(
                vec![1, 2, 3, 2, 1],
                vec![
                    vec![0, 1, 1],
                    vec![1, 2, 1],
                    vec![2, 3, 2],
                    vec![3, 4, 1],
                    vec![4, 4, 1]
                ]
            ),
            4
        );
    }

    #[test]
    fn example4() {
        assert_eq!(
            min_zero_array(
                vec![1, 2, 3, 2, 6],
                vec![
                    vec![0, 1, 1],
                    vec![0, 2, 1],
                    vec![1, 4, 2],
                    vec![4, 4, 4],
                    vec![3, 4, 1],
                    vec![4, 4, 5]
                ]
            ),
            4
        );
    }
}
