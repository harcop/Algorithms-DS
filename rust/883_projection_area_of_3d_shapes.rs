/// LeetCode #883 - Projection Area of 3d Shapes
fn projection_area(grid: Vec<Vec<i32>>) -> i32 {
    let n = grid.len();
    if n == 0 {
        return 0;
    }
    let m = grid[0].len();
    let mut xy = 0;
    let mut xz = vec![0; n];
    let mut yz = vec![0; m];

    for i in 0..n {
        for j in 0..m {
            let v = grid[i][j];
            if v > 0 {
                xy += 1;
            }
            xz[i] = xz[i].max(v);
            yz[j] = yz[j].max(v);
        }
    }
    xy + xz.iter().sum::<i32>() + yz.iter().sum::<i32>()
}

fn main() {
    println!("{}", projection_area(vec![vec![1, 2], vec![3, 4]]));
}

#[cfg(test)]
mod tests {
    use super::projection_area;

    #[test]
    fn example_one() {
        assert_eq!(projection_area(vec![vec![1, 2], vec![3, 4]]), 17);
    }
}
