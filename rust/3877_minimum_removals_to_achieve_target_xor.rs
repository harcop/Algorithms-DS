/// LeetCode #3877 - Minimum Removals to Achieve Target XOR
fn min_removals(nums: Vec<i32>, target: i32) -> i32 {
    let mx = *nums.iter().max().unwrap();
    let m = if mx == 0 {
        0
    } else {
        32 - mx.leading_zeros()
    } as usize;
    if (1 << m) <= target {
        return -1;
    }
    let n = nums.len();
    let width = 1usize << m;
    let mut f = vec![vec![i32::MIN / 2; width]; n + 1];
    f[0][0] = 0;
    for i in 1..=n {
        let x = nums[i - 1] as usize;
        for j in 0..width {
            f[i][j] = f[i - 1][j].max(f[i - 1][j ^ x] + 1);
        }
    }
    if f[n][target as usize] < 0 {
        -1
    } else {
        n as i32 - f[n][target as usize]
    }
}

fn main() {
    println!("{}", min_removals(vec![1, 2, 3], 2));
}

#[cfg(test)]
mod tests {
    use super::min_removals;

    #[test]
    fn example1() {
        assert_eq!(min_removals(vec![1, 2, 3], 2), 1);
    }

    #[test]
    fn example2() {
        assert_eq!(min_removals(vec![2, 4], 1), -1);
    }

    #[test]
    fn example3() {
        assert_eq!(min_removals(vec![7], 7), 0);
    }
}
