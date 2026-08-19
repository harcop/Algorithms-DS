/// LeetCode #3277 - Maximum XOR Score Subarray Queries
fn maximum_subarray_xor(nums: Vec<i32>, queries: Vec<Vec<i32>>) -> Vec<i32> {
    let n = nums.len();
    let mut f = vec![vec![0; n]; n];
    let mut g = vec![vec![0; n]; n];
    for i in (0..n).rev() {
        f[i][i] = nums[i];
        g[i][i] = nums[i];
        for j in i + 1..n {
            f[i][j] = f[i][j - 1] ^ f[i + 1][j];
            g[i][j] = f[i][j].max(g[i][j - 1]).max(g[i + 1][j]);
        }
    }
    queries
        .into_iter()
        .map(|q| g[q[0] as usize][q[1] as usize])
        .collect()
}

fn main() {
    println!(
        "{:?}",
        maximum_subarray_xor(vec![2, 8, 4, 32, 16, 1], vec![vec![0, 2], vec![1, 4], vec![0, 5]])
    );
}

#[cfg(test)]
mod tests {
    use super::maximum_subarray_xor;

    #[test]
    fn example1() {
        assert_eq!(
            maximum_subarray_xor(
                vec![2, 8, 4, 32, 16, 1],
                vec![vec![0, 2], vec![1, 4], vec![0, 5]]
            ),
            vec![12, 60, 60]
        );
    }

    #[test]
    fn example2() {
        assert_eq!(
            maximum_subarray_xor(
                vec![0, 7, 3, 2, 8, 5, 1],
                vec![vec![0, 3], vec![1, 5], vec![2, 4], vec![2, 6], vec![5, 6]]
            ),
            vec![7, 14, 11, 14, 5]
        );
    }
}
