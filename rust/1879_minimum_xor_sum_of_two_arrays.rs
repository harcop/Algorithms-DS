/// LeetCode #1879 - Minimum XOR Sum of Two Arrays
const INF: i32 = i32::MAX / 2;

fn minimum_xor_sum(nums1: Vec<i32>, nums2: Vec<i32>) -> i32 {
    let n = nums2.len();
    let mut f = vec![vec![INF; 1 << n]; n + 1];
    f[0][0] = 0;
    for (i, &x) in nums1.iter().enumerate() {
        for j in 0..(1 << n) {
            for k in 0..n {
                if j >> k & 1 == 1 {
                    f[i + 1][j] = f[i + 1][j].min(f[i][j ^ (1 << k)] + (x ^ nums2[k]));
                }
            }
        }
    }
    f[n][(1 << n) - 1]
}

fn main() {
    println!("{}", minimum_xor_sum(vec![1, 2], vec![2, 3]));
}

#[cfg(test)]
mod tests {
    use super::minimum_xor_sum;

    #[test]
    fn example_one() {
        assert_eq!(minimum_xor_sum(vec![1, 2], vec![2, 3]), 2);
    }

    #[test]
    fn example_two() {
        assert_eq!(minimum_xor_sum(vec![1, 0, 3], vec![5, 3, 4]), 8);
    }
}
