/// LeetCode #3715 - Sum of Perfect Square Ancestors
use std::collections::HashMap;

fn linear_sieve(n: usize) -> Vec<usize> {
    let mut primes = Vec::new();
    let mut spf = vec![0; n + 1];
    for i in 2..=n {
        if spf[i] == 0 {
            spf[i] = i;
            primes.push(i);
        }
        for &p in &primes {
            if i * p > n || p > spf[i] {
                break;
            }
            spf[i * p] = p;
        }
    }
    spf
}

fn kernel(mut v: usize, spf: &[usize]) -> i32 {
    let mut x = 1i32;
    while v != 1 {
        let pfac = spf[v] as i32;
        if x % pfac == 0 {
            x /= pfac;
        } else {
            x *= pfac;
        }
        v /= spf[v];
    }
    x
}

fn sum_of_ancestors(n: i32, edges: Vec<Vec<i32>>, nums: Vec<i32>) -> i64 {
    let n = n as usize;
    let spf = linear_sieve(100_000);
    let mut adj = vec![Vec::new(); n];
    for e in edges {
        let u = e[0] as usize;
        let v = e[1] as usize;
        adj[u].push(v);
        adj[v].push(u);
    }

    let mut cnt: HashMap<i32, i32> = HashMap::new();
    let mut result = 0i64;

    fn dfs(
        u: usize,
        p: isize,
        adj: &[Vec<usize>],
        nums: &[i32],
        spf: &[usize],
        cnt: &mut HashMap<i32, i32>,
        result: &mut i64,
    ) {
        let x = kernel(nums[u] as usize, spf);
        *result += *cnt.get(&x).unwrap_or(&0) as i64;
        *cnt.entry(x).or_insert(0) += 1;
        for &nei in &adj[u] {
            if nei as isize == p {
                continue;
            }
            dfs(nei, u as isize, adj, nums, spf, cnt, result);
        }
        *cnt.get_mut(&x).unwrap() -= 1;
    }

    dfs(0, -1, &adj, &nums, &spf, &mut cnt, &mut result);
    result
}

fn main() {
    println!(
        "{}",
        sum_of_ancestors(3, vec![vec![0, 1], vec![1, 2]], vec![2, 8, 2])
    );
}

#[cfg(test)]
mod tests {
    use super::sum_of_ancestors;

    #[test]
    fn example1() {
        assert_eq!(
            sum_of_ancestors(3, vec![vec![0, 1], vec![1, 2]], vec![2, 8, 2]),
            3
        );
    }

    #[test]
    fn example2() {
        assert_eq!(
            sum_of_ancestors(3, vec![vec![0, 1], vec![0, 2]], vec![1, 2, 4]),
            1
        );
    }

    #[test]
    fn example3() {
        assert_eq!(
            sum_of_ancestors(
                4,
                vec![vec![0, 1], vec![0, 2], vec![1, 3]],
                vec![1, 2, 9, 4]
            ),
            2
        );
    }
}
