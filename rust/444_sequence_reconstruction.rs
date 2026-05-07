/// LeetCode #444 - Sequence Reconstruction (Kahn with unique next choice)
use std::collections::{HashMap, HashSet};

fn sequence_reconstruction(org: Vec<i32>, seqs: Vec<Vec<i32>>) -> bool {
    let mut nodes = HashSet::new();
    for s in &seqs {
        for &x in s {
            nodes.insert(x);
        }
    }
    if org.is_empty() {
        return seqs.is_empty() || nodes.is_empty();
    }
    if nodes.len() != org.len() {
        return false;
    }
    for &x in &org {
        if !nodes.contains(&x) {
            return false;
        }
    }

    let mut g: HashMap<i32, HashSet<i32>> = HashMap::new();
    let mut indeg: HashMap<i32, usize> = HashMap::new();
    for &x in &nodes {
        indeg.insert(x, 0);
    }
    for s in seqs {
        if s.is_empty() {
            continue;
        }
        for w in s.windows(2) {
            let a = w[0];
            let b = w[1];
            let ins = g.entry(a).or_insert_with(HashSet::new).insert(b);
            if ins {
                *indeg.entry(b).or_insert(0) += 1;
            }
        }
    }

    for need in org {
        let zero: Vec<i32> = indeg
            .iter()
            .filter_map(|(&v, &d)| if d == 0 { Some(v) } else { None })
            .collect();
        if zero.len() != 1 || zero[0] != need {
            return false;
        }
        let u = need;
        indeg.remove(&u);
        if let Some(nei) = g.remove(&u) {
            for v in nei {
                indeg.entry(v).and_modify(|d| *d -= 1);
            }
        }
    }

    indeg.is_empty()
}

fn main() {
    println!(
        "{}",
        sequence_reconstruction(vec![1, 2, 3], vec![vec![1, 2], vec![1, 3]])
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn examples() {
        assert!(!sequence_reconstruction(
            vec![1, 2, 3],
            vec![vec![1, 2], vec![1, 3]]
        ));
        assert!(!sequence_reconstruction(
            vec![1, 2, 3],
            vec![vec![1, 2]]
        ));
    }
}
