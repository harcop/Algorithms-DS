/// LeetCode #3939 - Count Non Adjacent Subsets in a Rooted Tree
const MOD: i64 = 1_000_000_007;

fn convolve(a: &[i64], b: &[i64]) -> Vec<i64> {
    let k = a.len();
    let mut c = vec![0i64; k];
    for i in 0..k {
        if a[i] == 0 {
            continue;
        }
        for j in 0..k {
            let idx = (i + j) % k;
            c[idx] = (c[idx] + a[i] * b[j]) % MOD;
        }
    }
    c
}

fn count_subsets(parent: Vec<i32>, nums: Vec<i32>, k: i32) -> i32 {
    let n = parent.len();
    let k = k as usize;
    let mut children = vec![Vec::new(); n];
    for (i, &p) in parent.iter().enumerate().skip(1) {
        children[p as usize].push(i);
    }
    let mut take = vec![vec![0i64; k]; n];
    let mut skip = vec![vec![0i64; k]; n];
    for u in (0..n).rev() {
        skip[u][0] = 1;
        take[u][nums[u] as usize % k] = 1;
        for &v in &children[u] {
            let mut any = vec![0i64; k];
            for r in 0..k {
                any[r] = (skip[v][r] + take[v][r]) % MOD;
            }
            skip[u] = convolve(&skip[u], &any);
            take[u] = convolve(&take[u], &skip[v]);
        }
    }
    ((take[0][0] + skip[0][0] - 1 + MOD) % MOD) as i32
}

fn main() {
    println!("{}", count_subsets(vec![-1, 0, 1], vec![1, 2, 3], 3));
}

#[cfg(test)]
mod tests {
    use super::count_subsets;

    #[test]
    fn example1() {
        assert_eq!(count_subsets(vec![-1, 0, 1], vec![1, 2, 3], 3), 1);
    }

    #[test]
    fn example2() {
        assert_eq!(count_subsets(vec![-1, 0, 0, 0], vec![2, 1, 2, 1], 3), 2);
    }
}
