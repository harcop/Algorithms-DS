/// LeetCode #2397 - Maximum Rows Covered by Columns
fn maximum_rows(matrix: Vec<Vec<i32>>, num_select: i32) -> i32 {
    let m = matrix.len();
    let n = matrix[0].len();
    let mut rows = vec![0u32; m];
    for i in 0..m {
        for j in 0..n {
            if matrix[i][j] == 1 {
                rows[i] |= 1 << j;
            }
        }
    }
    let mut ans = 0;
    for mask in 0..(1u32 << n) {
        if mask.count_ones() != num_select as u32 {
            continue;
        }
        let covered = rows.iter().filter(|&&r| r & mask == r).count();
        ans = ans.max(covered as i32);
    }
    ans
}

fn main() {
    println!(
        "{}",
        maximum_rows(
            vec![vec![0, 0, 0], vec![1, 0, 1], vec![0, 1, 1], vec![0, 0, 1]],
            2
        )
    );
}

#[cfg(test)]
mod tests {
    use super::maximum_rows;

    #[test]
    fn example_one() {
        assert_eq!(
            maximum_rows(
                vec![vec![0, 0, 0], vec![1, 0, 1], vec![0, 1, 1], vec![0, 0, 1]],
                2
            ),
            3
        );
    }

    #[test]
    fn example_two() {
        assert_eq!(maximum_rows(vec![vec![1], vec![0]], 1), 2);
    }
}
