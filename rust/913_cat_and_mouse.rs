/// LeetCode #913 - Cat and Mouse
use std::collections::HashMap;

fn cat_mouse_game(graph: Vec<Vec<i32>>) -> i32 {
    let n = graph.len() as i32;
    let mut memo: std::collections::HashMap<(i32, i32, i32), i32> =
        std::collections::HashMap::new();

    fn dfs(
        mouse: i32,
        cat: i32,
        t: i32,
        graph: &Vec<Vec<i32>>,
        n: i32,
        memo: &mut HashMap<(i32, i32, i32), i32>,
    ) -> i32 {
        const DRAW: i32 = 0;
        const MOUSE_WIN: i32 = 1;
        const CAT_WIN: i32 = 2;
        if t == 2 * n {
            return DRAW;
        }
        if mouse == 0 {
            return MOUSE_WIN;
        }
        if mouse == cat {
            return CAT_WIN;
        }
        if let Some(&v) = memo.get(&(mouse, cat, t)) {
            return v;
        }

        let ans = if t % 2 == 0 {
            let mut cat_wins_all = true;
            let mut out = DRAW;
            for &x in &graph[mouse as usize] {
                let r = dfs(x, cat, t + 1, graph, n, memo);
                if r == MOUSE_WIN {
                    out = MOUSE_WIN;
                    break;
                }
                cat_wins_all &= r == CAT_WIN;
            }
            if out == MOUSE_WIN {
                MOUSE_WIN
            } else if cat_wins_all {
                CAT_WIN
            } else {
                DRAW
            }
        } else {
            let mut mouse_wins_all = true;
            let mut out = DRAW;
            for &x in &graph[cat as usize] {
                if x == 0 {
                    continue;
                }
                let r = dfs(mouse, x, t + 1, graph, n, memo);
                if r == CAT_WIN {
                    out = CAT_WIN;
                    break;
                }
                mouse_wins_all &= r == MOUSE_WIN;
            }
            if out == CAT_WIN {
                CAT_WIN
            } else if mouse_wins_all {
                MOUSE_WIN
            } else {
                DRAW
            }
        };

        memo.insert((mouse, cat, t), ans);
        ans
    }

    dfs(1, 2, 0, &graph, n, &mut memo)
}

fn main() {
    println!(
        "{}",
        cat_mouse_game(vec![
            vec![2, 5],
            vec![3],
            vec![0, 4, 5],
            vec![1, 4, 5],
            vec![2, 3],
            vec![0, 2, 3],
        ])
    );
}

#[cfg(test)]
mod tests {
    use super::cat_mouse_game;

    #[test]
    fn example_one() {
        let g = vec![
            vec![2, 5],
            vec![3],
            vec![0, 4, 5],
            vec![1, 4, 5],
            vec![2, 3],
            vec![0, 2, 3],
        ];
        assert_eq!(cat_mouse_game(g), 0);
    }

    #[test]
    fn example_two() {
        let g = vec![vec![1, 3], vec![0], vec![1], vec![0]];
        assert_eq!(cat_mouse_game(g), 1);
    }
}
