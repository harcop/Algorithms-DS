/// LeetCode #2536 - Increment Submatrices by One
fn range_add_queries(n: i32, queries: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    let n = n as usize;
    let mut mat = vec![vec![0i32; n]; n];
    for q in queries {
        let (x1, y1, x2, y2) = (q[0] as usize, q[1] as usize, q[2] as usize, q[3] as usize);
        mat[x1][y1] += 1;
        if x2 + 1 < n {
            mat[x2 + 1][y1] -= 1;
        }
        if y2 + 1 < n {
            mat[x1][y2 + 1] -= 1;
        }
        if x2 + 1 < n && y2 + 1 < n {
            mat[x2 + 1][y2 + 1] += 1;
        }
    }
    for i in 0..n {
        for j in 0..n {
            if i > 0 {
                mat[i][j] += mat[i - 1][j];
            }
            if j > 0 {
                mat[i][j] += mat[i][j - 1];
            }
            if i > 0 && j > 0 {
                mat[i][j] -= mat[i - 1][j - 1];
            }
        }
    }
    mat
}

fn main() {
    println!("{:?}", range_add_queries(3, vec![vec![1, 1, 2, 2], vec![0, 0, 1, 1]]));
}

#[cfg(test)]
mod tests {
    use super::range_add_queries;

    #[test]
    fn example_one() {
        assert_eq!(
            range_add_queries(3, vec![vec![1, 1, 2, 2], vec![0, 0, 1, 1]]),
            vec![vec![1, 1, 0], vec![1, 2, 1], vec![0, 1, 1]]
        );
    }

    #[test]
    fn example_two() {
        assert_eq!(
            range_add_queries(2, vec![vec![0, 0, 1, 1]]),
            vec![vec![1, 1], vec![1, 1]]
        );
    }
}
