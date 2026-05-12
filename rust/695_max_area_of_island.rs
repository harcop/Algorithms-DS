/// LeetCode #695 - Max Area of Island
fn max_area_of_island(grid: Vec<Vec<i32>>) -> i32 {
    let m = grid.len(); let n = grid[0].len();
    let mut g = grid;
    fn dfs(g: &mut Vec<Vec<i32>>, i: i32, j: i32) -> i32 {
        if i < 0 || j < 0 || i >= g.len() as i32 || j >= g[0].len() as i32 || g[i as usize][j as usize] == 0 { return 0; }
        g[i as usize][j as usize] = 0;
        1 + dfs(g, i+1, j) + dfs(g, i-1, j) + dfs(g, i, j+1) + dfs(g, i, j-1)
    }
    let mut best = 0i32;
    for i in 0..m as i32 {
        for j in 0..n as i32 {
            best = best.max(dfs(&mut g, i, j));
        }
    }
    best
}

fn main() {
    println!("{}", max_area_of_island(vec![vec![1,1,0],vec![0,1,0]]));
}

#[cfg(test)]
mod tests {
    use super::max_area_of_island;

    #[test]
    fn example_one() {
        assert_eq!(max_area_of_island(vec![vec![1,1,0],vec![0,1,0]]), 3);
    }
}
