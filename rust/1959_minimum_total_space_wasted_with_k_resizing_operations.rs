/// LeetCode #1959 - Minimum Total Space Wasted With K Resizing Operations
const INF: i32 = i32::MAX / 2;

fn min_space_wasted_k_resizing(nums: Vec<i32>, k: i32) -> i32 {
    let k = k as usize + 1;
    let n = nums.len();
    let mut g = vec![vec![0i32; n]; n];
    for i in 0..n {
        let mut s = 0i32;
        let mut mx = 0i32;
        for j in i..n {
            s += nums[j];
            mx = mx.max(nums[j]);
            g[i][j] = mx * (j - i + 1) as i32 - s;
        }
    }

    let mut f = vec![vec![INF; k + 1]; n + 1];
    f[0][0] = 0;
    for i in 1..=n {
        for j in 1..=k {
            for h in 0..i {
                f[i][j] = f[i][j].min(f[h][j - 1] + g[h][i - 1]);
            }
        }
    }
    f[n][k]
}

fn main() {
    println!("{}", min_space_wasted_k_resizing(vec![10, 20], 0));
}

#[cfg(test)]
mod tests {
    use super::min_space_wasted_k_resizing;

    #[test]
    fn example_one() {
        assert_eq!(min_space_wasted_k_resizing(vec![10, 20], 0), 10);
    }

    #[test]
    fn example_two() {
        assert_eq!(min_space_wasted_k_resizing(vec![10, 20, 30], 1), 10);
    }

    #[test]
    fn example_three() {
        assert_eq!(
            min_space_wasted_k_resizing(vec![10, 20, 15, 30, 20], 2),
            15
        );
    }
}
