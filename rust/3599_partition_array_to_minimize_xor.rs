/// LeetCode #3599 - Partition Array to Minimize XOR
fn min_xor(nums: Vec<i32>, k: i32) -> i32 {
    let n = nums.len();
    let k = k as usize;
    let mut g = vec![0; n + 1];
    for i in 0..n {
        g[i + 1] = g[i] ^ nums[i];
    }
    const INF: i32 = i32::MAX;
    let mut f = vec![vec![INF; k + 1]; n + 1];
    f[0][0] = 0;
    for i in 1..=n {
        for j in 1..=i.min(k) {
            for h in (j - 1)..i {
                let mx = f[h][j - 1].max(g[i] ^ g[h]);
                if mx < f[i][j] {
                    f[i][j] = mx;
                }
            }
        }
    }
    f[n][k]
}

fn main() {
    println!("{}", min_xor(vec![1, 2, 3], 2));
}

#[cfg(test)]
mod tests {
    use super::min_xor;

    #[test]
    fn example1() {
        assert_eq!(min_xor(vec![1, 2, 3], 2), 1);
    }

    #[test]
    fn example2() {
        assert_eq!(min_xor(vec![2, 3, 3, 2], 3), 2);
    }

    #[test]
    fn example3() {
        assert_eq!(min_xor(vec![1, 1, 2, 3, 1], 2), 0);
    }
}
