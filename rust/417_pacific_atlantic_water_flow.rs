/// LeetCode #417 - Pacific Atlantic Water Flow
fn pacific_atlantic(heights: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    let m = heights.len();
    if m == 0 {
        return vec![];
    }
    let n = heights[0].len();
    let mut pac = vec![vec![false; n]; m];
    let mut atl = vec![vec![false; n]; m];
    fn dfs(heights: &Vec<Vec<i32>>, seen: &mut Vec<Vec<bool>>, i: usize, j: usize, prev: i32) {
        let m = heights.len();
        let n = heights[0].len();
        if i >= m || j >= n || seen[i][j] || heights[i][j] < prev {
            return;
        }
        seen[i][j] = true;
        let h = heights[i][j];
        dfs(heights, seen, i.wrapping_sub(1), j, h);
        dfs(heights, seen, i + 1, j, h);
        dfs(heights, seen, i, j.wrapping_sub(1), h);
        dfs(heights, seen, i, j + 1, h);
    }
    for j in 0..n {
        dfs(&heights, &mut pac, 0, j, i32::MIN);
        dfs(&heights, &mut atl, m - 1, j, i32::MIN);
    }
    for i in 0..m {
        dfs(&heights, &mut pac, i, 0, i32::MIN);
        dfs(&heights, &mut atl, i, n - 1, i32::MIN);
    }
    let mut out = vec![];
    for i in 0..m {
        for j in 0..n {
            if pac[i][j] && atl[i][j] {
                out.push(vec![i as i32, j as i32]);
            }
        }
    }
    out
}

fn main() {
    println!(
        "{:?}",
        pacific_atlantic(vec![
            vec![1, 2, 2, 3, 5],
            vec![3, 2, 3, 4, 4],
            vec![2, 4, 5, 3, 1],
            vec![6, 7, 1, 4, 5],
            vec![5, 1, 1, 2, 4],
        ])
    );
}

#[cfg(test)]
mod tests {
    use super::pacific_atlantic;

    #[test]
    fn example_one() {
        let mut v = pacific_atlantic(vec![
            vec![1, 2, 2, 3, 5],
            vec![3, 2, 3, 4, 4],
            vec![2, 4, 5, 3, 1],
            vec![6, 7, 1, 4, 5],
            vec![5, 1, 1, 2, 4],
        ]);
        v.sort_unstable();
        let mut e = vec![
            vec![0, 4],
            vec![1, 3],
            vec![1, 4],
            vec![2, 2],
            vec![3, 0],
            vec![3, 1],
            vec![4, 0],
        ];
        e.sort_unstable();
        assert_eq!(v, e);
    }
}
