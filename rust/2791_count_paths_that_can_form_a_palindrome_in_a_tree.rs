/// LeetCode #2791 - Count Paths That Can Form a Palindrome in a Tree
use std::collections::HashMap;

fn count_palindrome_paths(parent: Vec<i32>, s: String) -> i64 {
    let n = parent.len();
    let bytes = s.as_bytes();
    let mut g: Vec<Vec<(usize, i32)>> = vec![Vec::new(); n];
    for i in 1..n {
        let p = parent[i] as usize;
        g[p].push((i, 1 << (bytes[i] - b'a') as i32));
    }
    let mut cnt: HashMap<i32, i64> = HashMap::new();
    cnt.insert(0, 1);
    let mut ans = 0i64;
    fn dfs(
        i: usize,
        xor: i32,
        g: &[Vec<(usize, i32)>],
        cnt: &mut HashMap<i32, i64>,
        ans: &mut i64,
    ) {
        for &(j, v) in &g[i] {
            let x = xor ^ v;
            *ans += *cnt.get(&x).unwrap_or(&0);
            for k in 0..26 {
                *ans += *cnt.get(&(x ^ (1 << k))).unwrap_or(&0);
            }
            *cnt.entry(x).or_insert(0) += 1;
            dfs(j, x, g, cnt, ans);
        }
    }
    dfs(0, 0, &g, &mut cnt, &mut ans);
    ans
}

fn main() {
    println!(
        "{}",
        count_palindrome_paths(vec![-1, 0, 0, 1, 1, 2], "acaabc".into())
    );
}

#[cfg(test)]
mod tests {
    use super::count_palindrome_paths;

    #[test]
    fn example_one() {
        assert_eq!(
            count_palindrome_paths(vec![-1, 0, 0, 1, 1, 2], "acaabc".into()),
            8
        );
    }

    #[test]
    fn example_two() {
        assert_eq!(
            count_palindrome_paths(vec![-1, 0, 0, 0, 0], "aaaaa".into()),
            10
        );
    }
}
