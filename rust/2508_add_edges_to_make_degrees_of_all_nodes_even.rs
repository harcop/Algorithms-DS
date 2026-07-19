/// LeetCode #2508 - Add Edges to Make Degrees of All Nodes Even
use std::collections::HashSet;

fn is_possible(n: i32, edges: Vec<Vec<i32>>) -> bool {
    let n = n as usize;
    let mut g = vec![HashSet::new(); n + 1];
    for e in edges {
        let a = e[0] as usize;
        let b = e[1] as usize;
        g[a].insert(b);
        g[b].insert(a);
    }

    let mut vs = Vec::new();
    for i in 1..=n {
        if g[i].len() % 2 == 1 {
            vs.push(i);
        }
    }

    match vs.len() {
        0 => true,
        2 => {
            let a = vs[0];
            let b = vs[1];
            if !g[a].contains(&b) {
                return true;
            }
            for c in 1..=n {
                if a != c && b != c && !g[a].contains(&c) && !g[b].contains(&c) {
                    return true;
                }
            }
            false
        }
        4 => {
            let (a, b, c, d) = (vs[0], vs[1], vs[2], vs[3]);
            (!g[a].contains(&b) && !g[c].contains(&d))
                || (!g[a].contains(&c) && !g[b].contains(&d))
                || (!g[a].contains(&d) && !g[b].contains(&c))
        }
        _ => false,
    }
}

fn main() {
    println!(
        "{}",
        is_possible(
            5,
            vec![
                vec![1, 2],
                vec![2, 3],
                vec![3, 4],
                vec![4, 2],
                vec![1, 4],
                vec![2, 5]
            ]
        )
    );
}

#[cfg(test)]
mod tests {
    use super::is_possible;

    #[test]
    fn example_one() {
        assert!(is_possible(
            5,
            vec![
                vec![1, 2],
                vec![2, 3],
                vec![3, 4],
                vec![4, 2],
                vec![1, 4],
                vec![2, 5]
            ]
        ));
    }

    #[test]
    fn example_two() {
        assert!(is_possible(4, vec![vec![1, 2], vec![3, 4]]));
    }

    #[test]
    fn example_three() {
        assert!(!is_possible(4, vec![vec![1, 2], vec![1, 3], vec![1, 4]]));
    }
}
