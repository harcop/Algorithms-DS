/// LeetCode #1719 - Number Of Ways To Reconstruct A Tree
use std::collections::{HashMap, HashSet};

fn check_ways(pairs: Vec<Vec<i32>>) -> i32 {
    let mut g: HashMap<i32, HashSet<i32>> = HashMap::new();
    for p in pairs {
        let (x, y) = (p[0], p[1]);
        g.entry(x).or_default().insert(y);
        g.entry(y).or_default().insert(x);
    }
    let n = g.len() as i32;
    let connected = |u: i32, v: i32| -> bool {
        u == v || g.get(&u).is_some_and(|s| s.contains(&v))
    };
    let Some(root) = g.keys().copied().find(|&r| {
        g.get(&r).is_some_and(|s| s.len() as i32 == n - 1)
    }) else {
        return 0;
    };

    let mut multi = false;
    fn dfs(
        u: i32,
        g: &HashMap<i32, HashSet<i32>>,
        ancestors: &[i32],
        seen: &mut HashSet<i32>,
        multi: &mut bool,
        connected: &impl Fn(i32, i32) -> bool,
    ) -> bool {
        for &a in ancestors {
            if !connected(u, a) {
                return false;
            }
        }
        seen.insert(u);
        let mut anc = ancestors.to_vec();
        anc.push(u);
        let deg_u = g.get(&u).map(|s| s.len()).unwrap_or(0);
        let mut children: Vec<i32> = g
            .get(&u)
            .into_iter()
            .flat_map(|s| s.iter().copied())
            .filter(|v| !seen.contains(v))
            .collect();
        children.sort_by_key(|v| -(g.get(v).map(|s| s.len()).unwrap_or(0) as i32));
        for v in children {
            if g.get(&v).map(|s| s.len()).unwrap_or(0) == deg_u {
                *multi = true;
            }
            if !dfs(v, g, &anc, seen, multi, connected) {
                return false;
            }
        }
        true
    }

    let mut seen = HashSet::new();
    if !dfs(root, &g, &[], &mut seen, &mut multi, &connected) {
        0
    } else if multi {
        2
    } else {
        1
    }
}
fn main() {
    println!("{}", check_ways(vec![vec![1, 2], vec![2, 3]]));
}
#[cfg(test)]
mod tests {
    use super::check_ways;
    #[test]
    fn example_one() {
        assert_eq!(check_ways(vec![vec![1, 2], vec![2, 3]]), 1);
    }
    #[test]
    fn example_two() {
        assert_eq!(check_ways(vec![vec![1, 2], vec![2, 3], vec![1, 3]]), 2);
    }
    #[test]
    fn example_three() {
        assert_eq!(
            check_ways(vec![vec![1, 2], vec![2, 3], vec![2, 4], vec![1, 5]]),
            0
        );
    }
}
