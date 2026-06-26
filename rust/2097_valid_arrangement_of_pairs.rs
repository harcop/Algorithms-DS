/// LeetCode #2097 - Valid Arrangement of Pairs
use std::collections::HashMap;

fn valid_arrangement(pairs: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    let mut g: HashMap<i32, Vec<i32>> = HashMap::new();
    let mut balance: HashMap<i32, i32> = HashMap::new();
    for p in &pairs {
        g.entry(p[0]).or_default().push(p[1]);
        *balance.entry(p[0]).or_default() += 1;
        *balance.entry(p[1]).or_default() -= 1;
    }

    let start = balance
        .iter()
        .find_map(|(&node, &diff)| (diff == 1).then_some(node))
        .unwrap_or(pairs[0][0]);

    fn dfs(u: i32, g: &mut HashMap<i32, Vec<i32>>, path: &mut Vec<i32>) {
        while let Some(v) = g.get_mut(&u).and_then(|next| next.pop()) {
            dfs(v, g, path);
        }
        path.push(u);
    }

    let mut path = Vec::with_capacity(pairs.len() + 1);
    dfs(start, &mut g, &mut path);
    path.reverse();

    path.windows(2).map(|w| vec![w[0], w[1]]).collect()
}

fn main() {
    println!(
        "{:?}",
        valid_arrangement(vec![vec![5, 1], vec![4, 5], vec![11, 9], vec![9, 4]])
    );
}

fn is_valid(original: &[Vec<i32>], arranged: &[Vec<i32>]) -> bool {
    if original.len() != arranged.len() {
        return false;
    }
    for i in 1..arranged.len() {
        if arranged[i - 1][1] != arranged[i][0] {
            return false;
        }
    }
    let mut need = HashMap::new();
    for p in original {
        *need.entry((p[0], p[1])).or_insert(0) += 1;
    }
    for p in arranged {
        let entry = need.entry((p[0], p[1])).or_insert(0);
        *entry -= 1;
    }
    need.values().all(|&v| v == 0)
}

#[cfg(test)]
mod tests {
    use super::{is_valid, valid_arrangement};

    #[test]
    fn example_one() {
        let pairs = vec![vec![5, 1], vec![4, 5], vec![11, 9], vec![9, 4]];
        let arranged = valid_arrangement(pairs.clone());
        assert!(is_valid(&pairs, &arranged));
    }

    #[test]
    fn example_two() {
        let pairs = vec![vec![1, 3], vec![3, 2], vec![2, 1]];
        let arranged = valid_arrangement(pairs.clone());
        assert!(is_valid(&pairs, &arranged));
    }

    #[test]
    fn example_three() {
        let pairs = vec![vec![1, 2], vec![1, 3], vec![2, 1]];
        let arranged = valid_arrangement(pairs.clone());
        assert!(is_valid(&pairs, &arranged));
    }
}
