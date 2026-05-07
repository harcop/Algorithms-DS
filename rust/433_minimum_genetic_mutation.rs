/// LeetCode #433 - Minimum Genetic Mutation
use std::collections::{HashSet, VecDeque};

fn min_mutation(start_gene: String, end_gene: String, bank: Vec<String>) -> i32 {
    let bank: HashSet<String> = bank.into_iter().collect();
    if !bank.contains(&end_gene) {
        return -1;
    }
    let mut q = VecDeque::new();
    q.push_back((start_gene.clone(), 0));
    let mut seen = HashSet::new();
    seen.insert(start_gene);
    while let Some((cur, d)) = q.pop_front() {
        if cur == end_gene {
            return d;
        }
        let cu = cur.as_bytes();
        for i in 0..cu.len() {
            for ch in b"ACGT" {
                if cu[i] == *ch {
                    continue;
                }
                let mut nxt = cu.to_vec();
                nxt[i] = *ch;
                let s = String::from_utf8(nxt).unwrap();
                if bank.contains(&s) && seen.insert(s.clone()) {
                    q.push_back((s, d + 1));
                }
            }
        }
    }
    -1
}

fn main() {
    println!(
        "{}",
        min_mutation(
            "AACCGGTT".into(),
            "AACCGGTA".into(),
            vec!["AACCGGTA".into(), "AACCGCTT".into(), "AACCCGGG".into()],
        )
    );
}

#[cfg(test)]
mod tests {
    use super::min_mutation;

    #[test]
    fn example_one() {
        assert_eq!(
            min_mutation(
                "AACCGGTT".into(),
                "AACCGGTA".into(),
                vec!["AACCGGTA".into(), "AACCGCTT".into(), "AACCCGGG".into()],
            ),
            1
        );
    }
}
