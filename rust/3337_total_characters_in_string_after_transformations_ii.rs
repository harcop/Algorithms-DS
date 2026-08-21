/// LeetCode #3337 - Total Characters in String After Transformations II
fn matmul(a: &[Vec<i64>], b: &[Vec<i64>]) -> Vec<Vec<i64>> {
    const MOD: i64 = 1_000_000_007;
    let n = a.len();
    let p = b.len();
    let q = b[0].len();
    let mut res = vec![vec![0i64; q]; n];
    for i in 0..n {
        for k in 0..p {
            if a[i][k] != 0 {
                for j in 0..q {
                    res[i][j] = (res[i][j] + a[i][k] * b[k][j]) % MOD;
                }
            }
        }
    }
    res
}

fn matpow(mut mat: Vec<Vec<i64>>, mut power: i32) -> Vec<Vec<i64>> {
    let m = mat.len();
    let mut res = vec![vec![0i64; m]; m];
    for i in 0..m {
        res[i][i] = 1;
    }
    while power > 0 {
        if power % 2 == 1 {
            res = matmul(&res, &mat);
        }
        mat = matmul(&mat, &mat);
        power /= 2;
    }
    res
}

fn length_after_transformations(s: String, t: i32, nums: Vec<i32>) -> i32 {
    const MOD: i64 = 1_000_000_007;
    let m = 26usize;
    let mut cnt = vec![0i64; m];
    for c in s.bytes() {
        cnt[(c - b'a') as usize] += 1;
    }
    let mut matrix = vec![vec![0i64; m]; m];
    for (i, &x) in nums.iter().enumerate() {
        for j in 1..=x {
            matrix[i][(i + j as usize) % m] = 1;
        }
    }
    let factor = matpow(matrix, t);
    let cnt = vec![cnt];
    let result = matmul(&cnt, &factor);
    (result[0].iter().sum::<i64>() % MOD) as i32
}

fn main() {
    println!(
        "{}",
        length_after_transformations(
            "abcyy".into(),
            2,
            vec![1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 2]
        )
    );
}

#[cfg(test)]
mod tests {
    use super::length_after_transformations;

    #[test]
    fn example1() {
        assert_eq!(
            length_after_transformations(
                "abcyy".into(),
                2,
                vec![1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 2]
            ),
            7
        );
    }

    #[test]
    fn example2() {
        assert_eq!(
            length_after_transformations(
                "azbk".into(),
                1,
                vec![2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2]
            ),
            8
        );
    }
}
