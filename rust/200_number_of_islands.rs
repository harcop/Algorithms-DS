/// LeetCode #200 - Number of Islands
fn num_islands(grid: Vec<Vec<char>>) -> i32 {
    let m = grid.len();
    if m == 0 {
        return 0;
    }
    let n = grid[0].len();
    let mut g = grid;
    let mut count = 0i32;

    fn dfs(g: &mut Vec<Vec<char>>, i: usize, j: usize, m: usize, n: usize) {
        if i >= m || j >= n || g[i][j] != '1' {
            return;
        }
        g[i][j] = '0';
        if i > 0 {
            dfs(g, i - 1, j, m, n);
        }
        if i + 1 < m {
            dfs(g, i + 1, j, m, n);
        }
        if j > 0 {
            dfs(g, i, j - 1, m, n);
        }
        if j + 1 < n {
            dfs(g, i, j + 1, m, n);
        }
    }

    for i in 0..m {
        for j in 0..n {
            if g[i][j] == '1' {
                count += 1;
                dfs(&mut g, i, j, m, n);
            }
        }
    }
    count
}

fn main() {
    let g = vec![
        vec!['1', '1', '1', '1', '0'],
        vec!['1', '1', '0', '1', '0'],
        vec!['1', '1', '0', '0', '0'],
        vec!['0', '0', '0', '0', '0'],
    ];
    println!("{}", num_islands(g));
}

#[cfg(test)]
mod tests {
    use super::num_islands;

    #[test]
    fn example_one() {
        let g = vec![
            vec!['1', '1', '1', '1', '0'],
            vec!['1', '1', '0', '1', '0'],
            vec!['1', '1', '0', '0', '0'],
            vec!['0', '0', '0', '0', '0'],
        ];
        assert_eq!(num_islands(g), 1);
    }

    #[test]
    fn example_two() {
        let g = vec![
            vec!['1', '1', '0', '0', '0'],
            vec!['1', '1', '0', '0', '0'],
            vec!['0', '0', '1', '0', '0'],
            vec!['0', '0', '0', '1', '1'],
        ];
        assert_eq!(num_islands(g), 3);
    }
}
