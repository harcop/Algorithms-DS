/// LeetCode #3316 - Find Maximum Removals From Source String
use std::collections::HashSet;

fn max_removals(source: String, pattern: String, target_indices: Vec<i32>) -> i32 {
    let source: Vec<u8> = source.into_bytes();
    let pattern: Vec<u8> = pattern.into_bytes();
    let m = source.len();
    let n = pattern.len();
    let s: HashSet<usize> = target_indices.into_iter().map(|i| i as usize).collect();
    const NEG: i32 = i32::MIN / 2;
    let mut f = vec![vec![NEG; n + 1]; m + 1];
    f[0][0] = 0;
    for i in 1..=m {
        let c = source[i - 1];
        for j in 0..=n {
            f[i][j] = f[i - 1][j] + if s.contains(&(i - 1)) { 1 } else { 0 };
            if j > 0 && c == pattern[j - 1] {
                f[i][j] = f[i][j].max(f[i - 1][j - 1]);
            }
        }
    }
    f[m][n].max(0)
}

fn main() {
    println!(
        "{}",
        max_removals("abbaa".into(), "aba".into(), vec![0, 1, 2])
    );
}

#[cfg(test)]
mod tests {
    use super::max_removals;

    #[test]
    fn example1() {
        assert_eq!(
            max_removals("abbaa".into(), "aba".into(), vec![0, 1, 2]),
            1
        );
    }

    #[test]
    fn example2() {
        assert_eq!(max_removals("bcda".into(), "d".into(), vec![0, 3]), 2);
    }

    #[test]
    fn example3() {
        assert_eq!(max_removals("dda".into(), "dda".into(), vec![0, 1, 2]), 0);
    }

    #[test]
    fn example4() {
        assert_eq!(
            max_removals("yeyeykyded".into(), "yeyyd".into(), vec![0, 2, 3, 4]),
            2
        );
    }
}
