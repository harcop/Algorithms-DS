/// LeetCode #3529 - Count Cells in Overlapping Horizontal and Vertical Substrings
fn prefix_function(p: &[u8]) -> Vec<usize> {
    let n = p.len();
    let mut pi = vec![0; n];
    for i in 1..n {
        let mut j = pi[i - 1];
        while j > 0 && p[i] != p[j] {
            j = pi[j - 1];
        }
        if p[i] == p[j] {
            j += 1;
        }
        pi[i] = j;
    }
    pi
}

fn kmp_starts(s: &[u8], p: &[u8]) -> Vec<usize> {
    let pi = prefix_function(p);
    let mut res = Vec::new();
    let mut j = 0;
    for (i, &c) in s.iter().enumerate() {
        while j > 0 && c != p[j] {
            j = pi[j - 1];
        }
        if c == p[j] {
            j += 1;
        }
        if j == p.len() {
            res.push(i + 1 - j);
            j = pi[j - 1];
        }
    }
    res
}

fn mark(s: &[u8], pattern: &[u8]) -> Vec<bool> {
    let n = s.len();
    let p = pattern.len();
    let mut diff = vec![0i32; n + 1];
    for idx in kmp_starts(s, pattern) {
        diff[idx] += 1;
        diff[idx + p] -= 1;
    }
    let mut marked = vec![false; n];
    let mut ps = 0i32;
    for i in 0..n {
        ps += diff[i];
        marked[i] = ps > 0;
    }
    marked
}

fn count_cells(grid: Vec<Vec<char>>, pattern: String) -> i32 {
    let m = grid.len();
    let n = grid[0].len();
    let pat = pattern.into_bytes();
    let mut h = Vec::with_capacity(m * n);
    for row in &grid {
        for &c in row {
            h.push(c as u8);
        }
    }
    let mut v = Vec::with_capacity(m * n);
    for c in 0..n {
        for r in 0..m {
            v.push(grid[r][c] as u8);
        }
    }
    let h_mark = mark(&h, &pat);
    let v_mark = mark(&v, &pat);
    let mut ans = 0;
    for r in 0..m {
        for c in 0..n {
            if h_mark[r * n + c] && v_mark[c * m + r] {
                ans += 1;
            }
        }
    }
    ans
}

fn main() {
    println!(
        "{}",
        count_cells(
            vec![
                vec!['a', 'a', 'c', 'c'],
                vec!['b', 'b', 'b', 'c'],
                vec!['a', 'a', 'b', 'a'],
                vec!['c', 'a', 'a', 'c'],
                vec!['a', 'a', 'b', 'a'],
            ],
            "abaca".into()
        )
    );
}

#[cfg(test)]
mod tests {
    use super::count_cells;

    #[test]
    fn example1() {
        assert_eq!(
            count_cells(
                vec![
                    vec!['a', 'a', 'c', 'c'],
                    vec!['b', 'b', 'b', 'c'],
                    vec!['a', 'a', 'b', 'a'],
                    vec!['c', 'a', 'a', 'c'],
                    vec!['a', 'a', 'b', 'a'],
                ],
                "abaca".into()
            ),
            1
        );
    }

    #[test]
    fn example2() {
        assert_eq!(
            count_cells(
                vec![
                    vec!['c', 'a', 'a', 'a'],
                    vec!['a', 'a', 'b', 'a'],
                    vec!['b', 'b', 'a', 'a'],
                    vec!['a', 'a', 'b', 'a'],
                ],
                "aba".into()
            ),
            4
        );
    }

    #[test]
    fn example3() {
        assert_eq!(count_cells(vec![vec!['a']], "a".into()), 1);
    }
}
