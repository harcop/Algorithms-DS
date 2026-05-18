/// LeetCode #1030 - Matrix Cells in Distance Order
fn all_cells_dist_order(rows: i32, cols: i32, r_center: i32, c_center: i32) -> Vec<Vec<i32>> {
    let mut cells: Vec<Vec<i32>> = Vec::new();
    for r in 0..rows {
        for c in 0..cols {
            cells.push(vec![r, c]);
        }
    }
    cells.sort_by_key(|p| {
        let dr = (p[0] - r_center).abs();
        let dc = (p[1] - c_center).abs();
        (dr + dc, p[0], p[1])
    });
    cells
}

fn main() {
    println!("{:?}", all_cells_dist_order(1, 2, 0, 0));
}

#[cfg(test)]
mod tests {
    use super::all_cells_dist_order;

    #[test]
    fn example_one() {
        assert_eq!(all_cells_dist_order(1, 2, 0, 0), vec![vec![0, 0], vec![0, 1]]);
    }
}
