/// LeetCode #694 - Number of Distinct Islands
use std::collections::HashSet;

fn num_distinct_islands(grid: Vec<Vec<i32>>) -> i32 {
    let m = grid.len();
    let n = grid[0].len();
    let mut g = grid;
    let mut shapes: HashSet<String> = HashSet::new();

    fn dfs(g: &mut Vec<Vec<i32>>, r: i32, c: i32, base_r: i32, base_c: i32, shape: &mut String) {
        if r < 0 || r >= g.len() as i32 || c < 0 || c >= g[0].len() as i32 {
            return;
        }
        if g[r as usize][c as usize] != 1 {
            return;
        }
        g[r as usize][c as usize] = 0;
        shape.push_str(&format!("{},{};", r - base_r, c - base_c));
        dfs(g, r + 1, c, base_r, base_c, shape);
        dfs(g, r - 1, c, base_r, base_c, shape);
        dfs(g, r, c + 1, base_r, base_c, shape);
        dfs(g, r, c - 1, base_r, base_c, shape);
    }

    for i in 0..m {
        for j in 0..n {
            if g[i][j] == 1 {
                let mut shape = String::new();
                dfs(&mut g, i as i32, j as i32, i as i32, j as i32, &mut shape);
                shapes.insert(shape);
            }
        }
    }
    shapes.len() as i32
}

fn main() {
    println!(
        "{}",
        num_distinct_islands(vec![
            vec![1, 1, 0, 0, 0],
            vec![1, 1, 0, 0, 0],
            vec![0, 0, 0, 1, 1],
            vec![0, 0, 0, 1, 1],
        ])
    );
}

#[cfg(test)]
mod tests {
    use super::num_distinct_islands;

    #[test]
    fn example_one() {
        assert_eq!(
            num_distinct_islands(vec![
                vec![1, 1, 0, 0, 0],
                vec![1, 1, 0, 0, 0],
                vec![0, 0, 0, 1, 1],
                vec![0, 0, 0, 1, 1],
            ]),
            1
        );
    }

    #[test]
    fn example_two() {
        assert_eq!(
            num_distinct_islands(vec![
                vec![1, 1, 0, 1, 1],
                vec![1, 0, 0, 0, 0],
                vec![0, 0, 0, 0, 1],
                vec![1, 1, 0, 1, 1],
            ]),
            3
        );
    }
}
