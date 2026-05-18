/// LeetCode #885 - Spiral Matrix III
fn spiral_matrix_iii(rows: i32, cols: i32, r_start: i32, c_start: i32) -> Vec<Vec<i32>> {
    let total = (rows * cols) as usize;
    let mut res = Vec::with_capacity(total);
    let dr = [0, 1, 0, -1];
    let dc = [1, 0, -1, 0];
    let mut r = r_start;
    let mut c = c_start;
    let mut dir = 0usize;
    let mut step_len = 1;

    if r >= 0 && r < rows && c >= 0 && c < cols {
        res.push(vec![r, c]);
    }

    while res.len() < total {
        for _ in 0..2 {
            let di = dir % 4;
            for _ in 0..step_len {
                r += dr[di];
                c += dc[di];
                if r >= 0 && r < rows && c >= 0 && c < cols {
                    res.push(vec![r, c]);
                }
                if res.len() == total {
                    return res;
                }
            }
            dir += 1;
        }
        step_len += 1;
    }
    res
}

fn main() {
    println!("{:?}", spiral_matrix_iii(1, 4, 0, 0));
}

#[cfg(test)]
mod tests {
    use super::spiral_matrix_iii;

    #[test]
    fn example_one() {
        assert_eq!(
            spiral_matrix_iii(1, 4, 0, 0),
            vec![vec![0, 0], vec![0, 1], vec![0, 2], vec![0, 3]]
        );
    }
}
