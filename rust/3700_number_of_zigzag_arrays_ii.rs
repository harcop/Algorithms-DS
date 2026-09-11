/// LeetCode #3700 - Number of ZigZag Arrays II
const MOD: i64 = 1_000_000_007;

fn mat_mul(a: &[Vec<i64>], b: &[Vec<i64>]) -> Vec<Vec<i64>> {
    let n = a.len();
    let mut res = vec![vec![0i64; n]; n];
    for i in 0..n {
        for k in 0..n {
            if a[i][k] == 0 {
                continue;
            }
            let aik = a[i][k];
            for j in 0..n {
                if b[k][j] == 0 {
                    continue;
                }
                res[i][j] = (res[i][j] + aik * b[k][j]) % MOD;
            }
        }
    }
    res
}

fn mat_vec_mul(mat: &[Vec<i64>], vec: &[i64]) -> Vec<i64> {
    let n = mat.len();
    let mut res = vec![0i64; n];
    for i in 0..n {
        let mut sum = 0i64;
        for j in 0..n {
            sum = (sum + mat[i][j] * vec[j]) % MOD;
        }
        res[i] = sum;
    }
    res
}

fn mat_pow(mut mat: Vec<Vec<i64>>, mut exp: i64) -> Vec<Vec<i64>> {
    let n = mat.len();
    let mut res = vec![vec![0i64; n]; n];
    for i in 0..n {
        res[i][i] = 1;
    }
    while exp > 0 {
        if exp & 1 == 1 {
            res = mat_mul(&res, &mat);
        }
        mat = mat_mul(&mat, &mat);
        exp >>= 1;
    }
    res
}

fn zig_zag_arrays(n: i32, l: i32, r: i32) -> i32 {
    let m = (r - l + 1) as usize;
    let size = 2 * m;
    let mut trans = vec![vec![0i64; size]; size];
    // from up[x] (m+x) to down[y] (y) where y < x
    for x in 0..m {
        for y in 0..x {
            trans[y][m + x] = 1;
        }
    }
    // from down[x] (x) to up[y] (m+y) where y > x
    for x in 0..m {
        for y in x + 1..m {
            trans[m + y][x] = 1;
        }
    }
    let power = mat_pow(trans, (n - 1) as i64);
    let mut init = vec![0i64; size];
    for i in 0..m {
        init[i] = 1;
        init[m + i] = 1;
    }
    let result = mat_vec_mul(&power, &init);
    let mut ans = 0i64;
    for v in result {
        ans = (ans + v) % MOD;
    }
    ans as i32
}

fn main() {
    println!("{}", zig_zag_arrays(3, 4, 5));
}

#[cfg(test)]
mod tests {
    use super::zig_zag_arrays;

    #[test]
    fn example1() {
        assert_eq!(zig_zag_arrays(3, 4, 5), 2);
    }

    #[test]
    fn example2() {
        assert_eq!(zig_zag_arrays(3, 1, 3), 10);
    }
}
