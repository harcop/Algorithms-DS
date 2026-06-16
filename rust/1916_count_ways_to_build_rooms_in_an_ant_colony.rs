/// LeetCode #1916 - Count Ways to Build Rooms in an Ant Colony
const MOD: i64 = 1_000_000_007;

fn ways_to_build_rooms(prev_room: Vec<i32>) -> i32 {
    let n = prev_room.len();
    let mut children = vec![vec![]; n];
    for i in 1..n {
        children[prev_room[i] as usize].push(i);
    }

    let mut fact = vec![1i64; n + 1];
    for i in 1..=n {
        fact[i] = fact[i - 1] * i as i64 % MOD;
    }

    fn inv(a: i64) -> i64 {
        let mut t = 0i64;
        let mut newt = 1i64;
        let mut r = MOD;
        let mut newr = a;
        while newr != 0 {
            let q = r / newr;
            (t, newt) = (newt, t - q * newt);
            (r, newr) = (newr, r - q * newr);
        }
        if r > 1 {
            return 1;
        }
        if t < 0 {
            t += MOD;
        }
        t
    }

    fn comb(n: usize, k: usize, fact: &[i64]) -> i64 {
        if k > n {
            return 0;
        }
        fact[n] * inv(fact[k] * fact[n - k] % MOD) % MOD
    }

    fn dfs(u: usize, children: &[Vec<usize>], fact: &[i64], ans: &mut i64) -> i64 {
        if children[u].is_empty() {
            return 1;
        }
        let mut nodes = 0i64;
        for &v in &children[u] {
            let cn = dfs(v, children, fact, ans);
            if nodes != 0 {
                *ans = *ans * comb((nodes + cn) as usize, cn as usize, fact) % MOD;
            }
            nodes += cn;
        }
        nodes + 1
    }

    let mut ans = 1i64;
    dfs(0, &children, &fact, &mut ans);
    ans as i32
}

fn main() {
    println!("{}", ways_to_build_rooms(vec![-1, 0, 1]));
}

#[cfg(test)]
mod tests {
    use super::ways_to_build_rooms;

    #[test]
    fn example_one() {
        assert_eq!(ways_to_build_rooms(vec![-1, 0, 1]), 1);
    }

    #[test]
    fn example_two() {
        assert_eq!(ways_to_build_rooms(vec![-1, 0, 0, 1, 2]), 6);
    }

    #[test]
    fn example_three() {
        assert_eq!(ways_to_build_rooms(vec![-1, 0, 0, 0, 1, 1, 1]), 180);
    }
}
