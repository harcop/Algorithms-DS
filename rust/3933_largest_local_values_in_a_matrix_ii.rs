/// LeetCode #3933 - Largest Local Values in a Matrix II
fn count_largest_local(matrix: Vec<Vec<i32>>) -> i32 {
    let n = matrix.len();
    let m = matrix[0].len();
    let logs = if m == 1 {
        1
    } else {
        (m as u32).ilog2() as usize + 1
    };
    let mut st = vec![vec![vec![0i32; m]; logs]; n];
    for i in 0..n {
        st[i][0].clone_from_slice(&matrix[i]);
        for k in 1..logs {
            let half = 1 << (k - 1);
            for j in 0..m {
                let mut v = st[i][k - 1][j];
                if j + half < m {
                    v = v.max(st[i][k - 1][j + half]);
                }
                st[i][k][j] = v;
            }
        }
    }
    let row_max = |i: usize, l: usize, r: usize| -> i32 {
        let len = r - l + 1;
        let k = (len as u32).ilog2() as usize;
        st[i][k][l].max(st[i][k][r + 1 - (1 << k)])
    };
    let mut ans = 0i32;
    for r in 0..n {
        for c in 0..m {
            let x = matrix[r][c];
            if x == 0 {
                continue;
            }
            let xu = x as usize;
            let left = c.saturating_sub(xu);
            let right = (c + xu).min(m - 1);
            let ir1 = r.saturating_sub(xu.saturating_sub(1));
            let ir2 = (r + xu).saturating_sub(1).min(n - 1);
            let mut mx = i32::MIN;
            if ir1 <= ir2 {
                for i in ir1..=ir2 {
                    mx = mx.max(row_max(i, left, right));
                }
            }
            let edge_cols = |c: usize, xu: usize| -> Option<(usize, usize)> {
                let cl = c.saturating_sub(xu.saturating_sub(1));
                let cr = (c + xu).saturating_sub(1).min(m - 1);
                if cl <= cr { Some((cl, cr)) } else { None }
            };
            if r >= xu {
                if let Some((cl, cr)) = edge_cols(c, xu) {
                    mx = mx.max(row_max(r - xu, cl, cr));
                }
            }
            if r + xu < n {
                if let Some((cl, cr)) = edge_cols(c, xu) {
                    mx = mx.max(row_max(r + xu, cl, cr));
                }
            }
            if mx <= x {
                ans += 1;
            }
        }
    }
    ans
}

fn main() {
    println!(
        "{}",
        count_largest_local(vec![vec![1, 2], vec![3, 4]])
    );
}

#[cfg(test)]
mod tests {
    use super::count_largest_local;

    #[test]
    fn example1() {
        let matrix = vec![
            vec![0, 0, 0, 0, 0, 0, 0],
            vec![0, 0, 0, 0, 0, 0, 0],
            vec![0, 0, 0, 0, 0, 0, 0],
            vec![0, 0, 0, 2, 0, 0, 0],
            vec![0, 0, 0, 0, 0, 0, 0],
            vec![0, 0, 0, 0, 0, 0, 0],
            vec![0, 0, 0, 0, 0, 0, 0],
        ];
        assert_eq!(count_largest_local(matrix), 1);
    }

    #[test]
    fn example2() {
        assert_eq!(count_largest_local(vec![vec![1, 2], vec![3, 4]]), 1);
    }

    #[test]
    fn example3() {
        assert_eq!(
            count_largest_local(vec![vec![1, 0, 1], vec![0, 1, 0], vec![1, 0, 1]]),
            5
        );
    }

    #[test]
    fn example4() {
        assert_eq!(count_largest_local(vec![vec![1, 1], vec![1, 1]]), 4);
    }
}
