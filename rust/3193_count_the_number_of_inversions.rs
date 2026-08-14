/// LeetCode #3193 - Count the Number of Inversions
fn number_of_permutations(n: i32, requirements: Vec<Vec<i32>>) -> i32 {
    const MOD: i32 = 1_000_000_007;
    let n = n as usize;
    let mut req = vec![-1i32; n];
    for r in &requirements {
        req[r[0] as usize] = r[1];
    }
    if req[0] > 0 {
        return 0;
    }
    req[0] = 0;
    let m = *req.iter().max().unwrap() as usize;
    let mut f = vec![vec![0i32; m + 1]; n];
    f[0][0] = 1;
    for i in 1..n {
        let (l, r) = if req[i] >= 0 {
            let v = req[i] as usize;
            (v, v)
        } else {
            (0, m)
        };
        for j in l..=r {
            for k in 0..=i.min(j) {
                f[i][j] = (f[i][j] + f[i - 1][j - k]) % MOD;
            }
        }
    }
    f[n - 1][req[n - 1] as usize]
}

fn main() {
    println!(
        "{}",
        number_of_permutations(3, vec![vec![2, 2], vec![0, 0]])
    );
}

#[cfg(test)]
mod tests {
    use super::number_of_permutations;

    #[test]
    fn example1() {
        assert_eq!(
            number_of_permutations(3, vec![vec![2, 2], vec![0, 0]]),
            2
        );
    }

    #[test]
    fn example2() {
        assert_eq!(
            number_of_permutations(3, vec![vec![2, 2], vec![1, 1], vec![0, 0]]),
            1
        );
    }

    #[test]
    fn example3() {
        assert_eq!(
            number_of_permutations(2, vec![vec![0, 0], vec![1, 0]]),
            1
        );
    }
}
