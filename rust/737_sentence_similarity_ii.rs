/// LeetCode #737 - Sentence Similarity II
use std::collections::HashMap;

struct DSU {
    p: Vec<usize>,
}

impl DSU {
    fn new(n: usize) -> Self {
        Self { p: (0..n).collect() }
    }
    fn find(&mut self, x: usize) -> usize {
        if self.p[x] != x {
            self.p[x] = self.find(self.p[x]);
        }
        self.p[x]
    }
    fn union(&mut self, a: usize, b: usize) {
        let pa = self.find(a);
        let pb = self.find(b);
        if pa != pb {
            self.p[pa] = pb;
        }
    }
}

fn are_sentences_similar_two(
    sentence1: Vec<String>,
    sentence2: Vec<String>,
    similar_pairs: Vec<Vec<String>>,
) -> bool {
    if sentence1.len() != sentence2.len() {
        return false;
    }
    let mut map: HashMap<String, usize> = HashMap::new();
    let mut next = 0usize;
    let mut reg = |w: &str| {
        *map.entry(w.to_string()).or_insert_with(|| {
            let i = next;
            next += 1;
            i
        })
    };
    for p in &similar_pairs {
        if p.len() == 2 {
            reg(&p[0]);
            reg(&p[1]);
        }
    }
    for w in sentence1.iter().chain(sentence2.iter()) {
        reg(w);
    }
    let mut uf = DSU::new(next);
    for p in &similar_pairs {
        if p.len() == 2 {
            let a = map[&p[0]];
            let b = map[&p[1]];
            uf.union(a, b);
        }
    }
    for i in 0..sentence1.len() {
        if sentence1[i] == sentence2[i] {
            continue;
        }
        let a = map[&sentence1[i]];
        let b = map[&sentence2[i]];
        if uf.find(a) != uf.find(b) {
            return false;
        }
    }
    true
}

fn main() {
    println!(
        "{}",
        are_sentences_similar_two(
            vec!["this".into(), "summer".into(), "is".into(), "too".into(), "hot".into()],
            vec!["this".into(), "summer".into(), "is".into(), "so".into(), "nice".into()],
            vec![
                vec!["nice".into(), "hot".into()],
                vec!["too".into(), "nice".into()],
                vec!["so".into(), "too".into()],
            ],
        )
    );
}

#[cfg(test)]
mod tests {
    use super::are_sentences_similar_two;

    #[test]
    fn example_one() {
        assert!(are_sentences_similar_two(
            vec!["an".into(), "extraordinary".into(), "meal".into()],
            vec!["one".into(), "good".into(), "dinner".into()],
            vec![
                vec!["great".into(), "good".into()],
                vec!["extraordinary".into(), "good".into()],
                vec!["well".into(), "good".into()],
                vec!["wonderful".into(), "good".into()],
                vec!["excellent".into(), "good".into()],
                vec!["fine".into(), "good".into()],
                vec!["nice".into(), "good".into()],
                vec!["any".into(), "one".into()],
                vec!["some".into(), "one".into()],
                vec!["unique".into(), "one".into()],
                vec!["an".into(), "one".into()],
                vec!["single".into(), "one".into()],
                vec!["the".into(), "one".into()],
                vec!["a".into(), "one".into()],
                vec!["easy".into(), "simple".into()],
                vec!["le".into(), "simple".into()],
                vec!["meal".into(), "dinner".into()],
            ],
        ));
    }
}
