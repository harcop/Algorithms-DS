/// LeetCode #1799 - Maximize Score After N Operations
fn gcd(mut a: i32, mut b: i32) -> i32 {
    while b != 0 {
        let t = a % b;
        a = b;
        b = t;
    }
    a
}

fn max_score(nums: Vec<i32>) -> i32 {
    let m = nums.len();
    let mut g = vec![vec![0i32; m]; m];
    for i in 0..m {
        for j in (i + 1)..m {
            g[i][j] = gcd(nums[i], nums[j]);
        }
    }
    let mut f = vec![0i32; 1 << m];
    for mask in 0usize..(1usize << m) {
        let cnt = mask.count_ones();
        if cnt % 2 != 0 {
            continue;
        }
        for i in 0..m {
            if mask & (1 << i) == 0 {
                continue;
            }
            for j in (i + 1)..m {
                if mask & (1 << j) == 0 {
                    continue;
                }
                let prev = mask ^ (1 << i) ^ (1 << j);
                f[mask] = f[mask].max(f[prev] + (cnt / 2) as i32 * g[i][j]);
            }
        }
    }
    f[(1 << m) - 1]
}

fn main() {
    println!("{}", max_score(vec![3, 4, 6, 8]));
}

#[cfg(test)]
mod tests {
    use super::max_score;

    #[test]
    fn example_one() {
        assert_eq!(max_score(vec![1, 2]), 1);
    }

    #[test]
    fn example_two() {
        assert_eq!(max_score(vec![3, 4, 6, 8]), 11);
    }

    #[test]
    fn example_three() {
        assert_eq!(max_score(vec![1, 2, 3, 4, 5, 6]), 14);
    }
}
