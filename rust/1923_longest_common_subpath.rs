/// LeetCode #1923 - Longest Common Subpath
use std::collections::HashMap;

const BASE: u128 = 133331;
const MOD: u128 = (1u128 << 64) + 1;

fn longest_common_subpath(_n: i32, paths: Vec<Vec<i32>>) -> i32 {
    let m = paths.len();
    let mx = paths.iter().map(|p| p.len()).max().unwrap();
    let mut pow = vec![0u128; mx + 1];
    pow[0] = 1;
    for i in 1..=mx {
        pow[i] = pow[i - 1] * BASE % MOD;
    }

    let mut hashes = Vec::with_capacity(m);
    for path in &paths {
        let k = path.len();
        let mut h = vec![0u128; k + 1];
        for (i, &x) in path.iter().enumerate() {
            h[i + 1] = (h[i] * BASE % MOD + x as u128) % MOD;
        }
        hashes.push(h);
    }

    let min_len = paths.iter().map(|p| p.len()).min().unwrap();
    let check = |k: usize| -> bool {
        let mut cnt: HashMap<u128, i32> = HashMap::new();
        for h in &hashes {
            let mut vis = std::collections::HashSet::new();
            for i in 1..=h.len() - k {
                let j = i + k - 1;
                let x = (h[j] + MOD - h[i - 1] * pow[j - i + 1] % MOD) % MOD;
                if vis.insert(x) {
                    *cnt.entry(x).or_insert(0) += 1;
                }
            }
        }
        cnt.values().copied().max().unwrap_or(0) == m as i32
    };

    let mut lo = 0usize;
    let mut hi = min_len;
    while lo < hi {
        let mid = lo + (hi - lo + 1) / 2;
        if check(mid) {
            lo = mid;
        } else {
            hi = mid - 1;
        }
    }
    lo as i32
}

fn main() {
    println!(
        "{}",
        longest_common_subpath(
            5,
            vec![vec![0, 1, 2, 3, 4], vec![2, 3, 4], vec![4, 0, 1, 2, 3]]
        )
    );
}

#[cfg(test)]
mod tests {
    use super::longest_common_subpath;

    #[test]
    fn example_one() {
        assert_eq!(
            longest_common_subpath(
                5,
                vec![vec![0, 1, 2, 3, 4], vec![2, 3, 4], vec![4, 0, 1, 2, 3]]
            ),
            2
        );
    }
}
