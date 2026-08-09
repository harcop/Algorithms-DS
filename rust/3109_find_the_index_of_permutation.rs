/// LeetCode #3109 - Find the Index of Permutation
struct BinaryIndexedTree {
    n: usize,
    c: Vec<i32>,
}

impl BinaryIndexedTree {
    fn new(n: usize) -> Self {
        Self {
            n,
            c: vec![0; n + 1],
        }
    }

    fn update(&mut self, mut x: usize, delta: i32) {
        while x <= self.n {
            self.c[x] += delta;
            x += x & x.wrapping_neg();
        }
    }

    fn query(&self, mut x: usize) -> i32 {
        let mut s = 0;
        while x > 0 {
            s += self.c[x];
            x -= x & x.wrapping_neg();
        }
        s
    }
}

fn get_permutation_index(perm: Vec<i32>) -> i32 {
    const MOD: i64 = 1_000_000_007;
    let n = perm.len();
    let mut tree = BinaryIndexedTree::new(n + 1);
    let mut f = vec![1i64; n];
    for i in 1..n {
        f[i] = f[i - 1] * i as i64 % MOD;
    }
    let mut ans = 0i64;
    for (i, &x) in perm.iter().enumerate() {
        let cnt = x as i64 - 1 - tree.query(x as usize) as i64;
        ans = (ans + cnt * f[n - i - 1] % MOD) % MOD;
        tree.update(x as usize, 1);
    }
    ans as i32
}

fn main() {
    println!("{}", get_permutation_index(vec![3, 1, 2]));
}

#[cfg(test)]
mod tests {
    use super::get_permutation_index;

    #[test]
    fn example1() {
        assert_eq!(get_permutation_index(vec![1, 2]), 0);
    }

    #[test]
    fn example2() {
        assert_eq!(get_permutation_index(vec![3, 1, 2]), 4);
    }
}
