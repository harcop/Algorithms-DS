/// LeetCode #1194 - Tournament Winners (SQL; Rust analogue)
use std::collections::HashMap;

fn tournament_winners(
    players: Vec<(i32, i32)>,
    matches: Vec<(i32, i32, i32, i32, i32)>,
) -> Vec<(i32, i32)> {
    let group: HashMap<i32, i32> = players.iter().cloned().collect();
    let mut score: HashMap<i32, i32> = players.iter().map(|(id, _)| (*id, 0)).collect();
    for (_, a, b, sa, sb) in matches {
        *score.entry(a).or_insert(0) += sa;
        *score.entry(b).or_insert(0) += sb;
    }
    let mut best: HashMap<i32, (i32, i32)> = HashMap::new();
    for (pid, sc) in score {
        let g = group[&pid];
        best.entry(g)
            .and_modify(|e| {
                if sc > e.1 || (sc == e.1 && pid < e.0) {
                    *e = (pid, sc);
                }
            })
            .or_insert((pid, sc));
    }
    let mut ans: Vec<(i32, i32)> = best.into_iter().map(|(g, (p, _))| (g, p)).collect();
    ans.sort();
    ans
}

fn main() {
    println!("ok");
}

#[cfg(test)]
mod tests {
    use super::tournament_winners;

    #[test]
    fn example() {
        let players = vec![
            (15, 1),
            (25, 1),
            (30, 1),
            (45, 1),
            (10, 2),
            (35, 2),
            (50, 2),
            (20, 3),
            (40, 3),
        ];
        let matches = vec![
            (1, 15, 45, 3, 0),
            (2, 30, 25, 1, 2),
            (3, 30, 15, 2, 0),
            (4, 40, 20, 5, 2),
            (5, 35, 50, 1, 1),
        ];
        assert_eq!(
            tournament_winners(players, matches),
            vec![(1, 15), (2, 35), (3, 40)]
        );
    }
}
