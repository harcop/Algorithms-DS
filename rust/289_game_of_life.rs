/// LeetCode #289 - Game of Life
fn game_of_life(board: &mut Vec<Vec<i32>>) {
    let m = board.len();
    let n = board[0].len();
    let orig = board.clone();
    for i in 0..m {
        for j in 0..n {
            let mut live = 0;
            for di in -1i32..=1 {
                for dj in -1i32..=1 {
                    if di == 0 && dj == 0 {
                        continue;
                    }
                    let ni = i as i32 + di;
                    let nj = j as i32 + dj;
                    if ni >= 0 && ni < m as i32 && nj >= 0 && nj < n as i32 {
                        live += orig[ni as usize][nj as usize];
                    }
                }
            }
            if orig[i][j] == 1 {
                board[i][j] = if live == 2 || live == 3 { 1 } else { 0 };
            } else {
                board[i][j] = if live == 3 { 1 } else { 0 };
            }
        }
    }
}

fn main() {
    let mut b = vec![vec![0, 1, 0], vec![0, 0, 1], vec![1, 1, 1], vec![0, 0, 0]];
    game_of_life(&mut b);
    println!("{:?}", b);
}

#[cfg(test)]
mod tests {
    use super::game_of_life;

    #[test]
    fn example_one() {
        let mut b = vec![vec![0, 1, 0], vec![0, 0, 1], vec![1, 1, 1], vec![0, 0, 0]];
        game_of_life(&mut b);
        assert_eq!(
            b,
            vec![vec![0, 0, 0], vec![1, 0, 1], vec![0, 1, 1], vec![0, 1, 0]]
        );
    }
}
