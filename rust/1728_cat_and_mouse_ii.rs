/// LeetCode #1728 - Cat and Mouse II
use std::collections::HashMap;

fn can_mouse_win(grid: Vec<String>, cat_jump: i32, mouse_jump: i32) -> bool {
    let m = grid.len();
    let n = grid[0].len();
    let mut mouse = 0usize;
    let mut cat = 0usize;
    let mut food = 0usize;
    for i in 0..m {
        for j in 0..n {
            match grid[i].as_bytes()[j] {
                b'M' => mouse = i * n + j,
                b'C' => cat = i * n + j,
                b'F' => food = i * n + j,
                _ => {}
            }
        }
    }
    let mouse_moves = build_moves(&grid, mouse_jump);
    let cat_moves = build_moves(&grid, cat_jump);
    let limit = m * n * 2;
    let mut memo: HashMap<(usize, usize, usize), bool> = HashMap::new();
    mouse_wins(
        mouse,
        cat,
        0,
        food,
        limit,
        &mouse_moves,
        &cat_moves,
        &mut memo,
    )
}

fn build_moves(grid: &[String], jump: i32) -> Vec<Vec<usize>> {
    let m = grid.len();
    let n = grid[0].len();
    let dirs = [(0, 1), (0, -1), (1, 0), (-1, 0)];
    let mut g = vec![vec![]; m * n];
    for i in 0..m {
        for j in 0..n {
            if grid[i].as_bytes()[j] == b'#' {
                continue;
            }
            let v = i * n + j;
            g[v].push(v);
            for (di, dj) in dirs {
                for step in 1..=jump {
                    let x = i as i32 + di * step;
                    let y = j as i32 + dj * step;
                    if x < 0 || y < 0 || x >= m as i32 || y >= n as i32 {
                        break;
                    }
                    if grid[x as usize].as_bytes()[y as usize] == b'#' {
                        break;
                    }
                    g[v].push(x as usize * n + y as usize);
                }
            }
        }
    }
    g
}

fn mouse_wins(
    mouse: usize,
    cat: usize,
    turn: usize,
    food: usize,
    limit: usize,
    mouse_moves: &[Vec<usize>],
    cat_moves: &[Vec<usize>],
    memo: &mut HashMap<(usize, usize, usize), bool>,
) -> bool {
    if mouse == cat {
        return false;
    }
    if cat == food {
        return false;
    }
    if mouse == food {
        return true;
    }
    if turn >= limit {
        return false;
    }
    if let Some(&v) = memo.get(&(mouse, cat, turn)) {
        return v;
    }
    let ans = if turn % 2 == 0 {
        mouse_moves[mouse]
            .iter()
            .any(|&nm| mouse_wins(nm, cat, turn + 1, food, limit, mouse_moves, cat_moves, memo))
    } else {
        cat_moves[cat].iter().all(|&nc| {
            if nc == food {
                true
            } else {
                mouse_wins(mouse, nc, turn + 1, food, limit, mouse_moves, cat_moves, memo)
            }
        })
    };
    memo.insert((mouse, cat, turn), ans);
    ans
}

fn main() {
    println!(
        "{}",
        can_mouse_win(
            vec!["####F".into(), "#C...".into(), "M....".into()],
            1,
            2,
        )
    );
}
#[cfg(test)]
mod tests {
    use super::can_mouse_win;
    #[test]
    fn example_one() {
        assert!(can_mouse_win(
            vec!["####F".into(), "#C...".into(), "M....".into()],
            1,
            2,
        ));
    }
    #[test]
    fn example_two() {
        assert!(can_mouse_win(vec!["M.C...F".into()], 1, 4));
    }
    #[test]
    fn example_three() {
        assert!(!can_mouse_win(vec!["M.C...F".into()], 1, 3));
    }
}
