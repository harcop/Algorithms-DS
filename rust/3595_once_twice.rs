/// LeetCode #3595 - Once Twice
fn once_twice(nums: Vec<i32>) -> Vec<i32> {
    let mut dp = [0i32; 3];
    dp[0] = !0;
    for &x in &nums {
        let mut ndp = [0i32; 3];
        for i in 0..3 {
            let prev = dp[(i + 2) % 3];
            ndp[i] = (x & prev) | (!x & dp[i]);
        }
        dp = ndp;
    }
    let mut dp2 = [0i32; 3];
    dp2[0] = !0;
    for &x in &nums {
        if (!x & dp[1]) != 0 || (x & dp[2]) != 0 {
            continue;
        }
        let mut ndp = [0i32; 3];
        for i in 0..3 {
            let prev = dp2[(i + 2) % 3];
            ndp[i] = (x & prev) | (!x & dp2[i]);
        }
        dp2 = ndp;
    }
    vec![dp2[1], (dp2[1] ^ dp[1]) | dp[2]]
}

fn main() {
    println!("{:?}", once_twice(vec![2, 2, 3, 2, 5, 5, 5, 7, 7]));
}

#[cfg(test)]
mod tests {
    use super::once_twice;

    #[test]
    fn example1() {
        assert_eq!(once_twice(vec![2, 2, 3, 2, 5, 5, 5, 7, 7]), vec![3, 7]);
    }

    #[test]
    fn example2() {
        assert_eq!(once_twice(vec![4, 4, 6, 4, 9, 9, 9, 6, 8]), vec![8, 6]);
    }
}
