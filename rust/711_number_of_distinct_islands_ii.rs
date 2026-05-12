/// LeetCode #711 - Number of Distinct Islands II
use std::collections::HashSet;

fn transform(r: i32, c: i32, k: usize) -> (i32, i32) {
    match k {
        0 => (r, c),
        1 => (r, -c),
        2 => (-r, c),
        3 => (-r, -c),
        4 => (c, r),
        5 => (c, -r),
        6 => (-c, r),
        _ => (-c, -r),
    }
}

fn canonical(cells: &[(i32, i32)]) -> Vec<(i32, i32)> {
    let mut best: Option<Vec<(i32, i32)>> = None;
    for k in 0..8 {
        let mut shape: Vec<(i32, i32)> = cells.iter().map(|&(r, c)| transform(r, c, k)).collect();
        shape.sort();
        let (or, oc) = shape[0];
        let normalized: Vec<(i32, i32)> = shape.iter().map(|&(r, c)| (r - or, c - oc)).collect();
        match &best {
            None => best = Some(normalized),
            Some(b) if &normalized < b => best = Some(normalized),
            _ => {}
        }
    }
    best.unwrap()
}

fn num_distinct_islands2(grid: Vec<Vec<i32>>) -> i32 {
    let m = grid.len();
    let n = grid[0].len();
    let mut g = grid;
    let mut shapes: HashSet<Vec<(i32, i32)>> = HashSet::new();

    fn dfs(g: &mut Vec<Vec<i32>>, r: i32, c: i32, cells: &mut Vec<(i32, i32)>) {
        if r < 0 || r >= g.len() as i32 || c < 0 || c >= g[0].len() as i32 {
            return;
        }
        if g[r as usize][c as usize] != 1 {
            return;
        }
        g[r as usize][c as usize] = 0;
        cells.push((r, c));
        dfs(g, r + 1, c, cells);
        dfs(g, r - 1, c, cells);
        dfs(g, r, c + 1, cells);
        dfs(g, r, c - 1, cells);
    }

    for i in 0..m {
        for j in 0..n {
            if g[i][j] == 1 {
                let mut cells = vec![];
                dfs(&mut g, i as i32, j as i32, &mut cells);
                shapes.insert(canonical(&cells));
            }
        }
    }
    shapes.len() as i32
}

fn main() {
    println!(
        "{}",
        num_distinct_islands2(vec![
            vec![1, 1, 0, 0, 0],
            vec![1, 0, 0, 0, 0],
            vec![0, 0, 0, 0, 1],
            vec![0, 0, 0, 1, 1],
        ])
    );
}

#[cfg(test)]
mod tests {
    use super::num_distinct_islands2;

    #[test]
    fn example_one() {
        assert_eq!(
            num_distinct_islands2(vec![
                vec![1, 1, 0, 0, 0],
                vec![1, 0, 0, 0, 0],
                vec![0, 0, 0, 0, 1],
                vec![0, 0, 0, 1, 1],
            ]),
            1
        );
    }

    #[test]
    fn example_two() {
        assert_eq!(
            num_distinct_islands2(vec![
                vec![1, 1, 1],
                vec![0, 0, 0],
                vec![1, 1, 0],
            ]),
            2
        );
    }
}
