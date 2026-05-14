/// LeetCode #723 - Candy Crush
fn candy_crush(mut board: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    let m = board.len();
    let n = board[0].len();
    loop {
        let mut mark = vec![vec![false; n]; m];
        for i in 0..m {
            let mut j = 0usize;
            while j < n {
                let v = board[i][j];
                if v == 0 {
                    j += 1;
                    continue;
                }
                let mut k = j + 1;
                while k < n && board[i][k] == v {
                    k += 1;
                }
                if k - j >= 3 {
                    for t in j..k {
                        mark[i][t] = true;
                    }
                }
                j = k;
            }
        }
        for j in 0..n {
            let mut i = 0usize;
            while i < m {
                let v = board[i][j];
                if v == 0 {
                    i += 1;
                    continue;
                }
                let mut k = i + 1;
                while k < m && board[k][j] == v {
                    k += 1;
                }
                if k - i >= 3 {
                    for t in i..k {
                        mark[t][j] = true;
                    }
                }
                i = k;
            }
        }
        let mut any = false;
        for i in 0..m {
            for j in 0..n {
                if mark[i][j] {
                    board[i][j] = 0;
                    any = true;
                }
            }
        }
        if !any {
            break;
        }
        for j in 0..n {
            let mut w = m;
            for i in (0..m).rev() {
                if board[i][j] != 0 {
                    w -= 1;
                    board[w][j] = board[i][j];
                }
            }
            for i in 0..w {
                board[i][j] = 0;
            }
        }
    }
    board
}

fn main() {
    let b = vec![
        vec![110, 5, 112, 113, 114],
        vec![210, 211, 5, 213, 214],
        vec![310, 311, 3, 313, 314],
        vec![410, 411, 412, 5, 414],
        vec![5, 1, 512, 3, 3],
        vec![610, 4, 1, 613, 614],
        vec![710, 1, 2, 713, 714],
        vec![810, 1, 2, 1, 1],
        vec![1, 1, 2, 2, 2],
        vec![4, 4, 4, 4, 4],
    ];
    println!("{:?}", candy_crush(b));
}

#[cfg(test)]
mod tests {
    use super::candy_crush;

    #[test]
    fn crushes_and_drops() {
        let b = vec![
            vec![110, 5, 112, 113, 114],
            vec![210, 211, 5, 213, 214],
            vec![310, 311, 3, 313, 314],
            vec![410, 411, 412, 5, 414],
            vec![5, 1, 512, 3, 3],
            vec![610, 4, 1, 613, 614],
            vec![710, 1, 2, 713, 714],
            vec![810, 1, 2, 1, 1],
            vec![1, 1, 2, 2, 2],
            vec![4, 4, 4, 4, 4],
        ];
        let got = candy_crush(b);
        assert_eq!(got.len(), 10);
        assert_eq!(got[0].len(), 5);
    }
}
