/// LeetCode #1258 - Synonymous Sentences
use std::collections::{HashMap, HashSet};

struct UnionFind {
    parent: Vec<usize>,
}

impl UnionFind {
    fn new(n: usize) -> Self {
        Self {
            parent: (0..n).collect(),
        }
    }

    fn find(&mut self, x: usize) -> usize {
        if self.parent[x] != x {
            self.parent[x] = self.find(self.parent[x]);
        }
        self.parent[x]
    }

    fn union(&mut self, a: usize, b: usize) {
        let ra = self.find(a);
        let rb = self.find(b);
        if ra != rb {
            self.parent[rb] = ra;
        }
    }
}

fn generate_sentences(synonyms: Vec<Vec<String>>, text: String) -> Vec<String> {
    let words: Vec<String> = text.split_whitespace().map(String::from).collect();
    let n = words.len();
    let mut mp: HashMap<String, usize> = HashMap::new();
    for (i, w) in words.iter().enumerate() {
        mp.insert(w.clone(), i);
    }
    let mut uf = UnionFind::new(n);
    for pair in &synonyms {
        if let (Some(&a), Some(&b)) = (mp.get(&pair[0]), mp.get(&pair[1])) {
            uf.union(a, b);
        }
    }
    let mut groups: HashMap<usize, HashSet<String>> = HashMap::new();
    for (i, w) in words.iter().enumerate() {
        groups.entry(uf.find(i)).or_default().insert(w.clone());
    }
    for pair in synonyms {
        if let (Some(&a), Some(&b)) = (mp.get(&pair[0]), mp.get(&pair[1])) {
            let root = uf.find(a);
            groups.entry(root).or_default().insert(pair[0]);
            groups.entry(root).or_default().insert(pair[1]);
        }
    }
    let mut slots: Vec<Vec<String>> = vec![];
    for (i, _) in words.iter().enumerate() {
        let root = uf.find(i);
        let mut opts: Vec<String> = groups[&root].iter().cloned().collect();
        opts.sort();
        slots.push(opts);
    }
    let mut result = vec![];
    fn dfs(i: usize, cur: &mut Vec<String>, slots: &[Vec<String>], out: &mut Vec<String>) {
        if i == slots.len() {
            out.push(cur.join(" "));
            return;
        }
        for w in &slots[i] {
            cur.push(w.clone());
            dfs(i + 1, cur, slots, out);
            cur.pop();
        }
    }
    let mut cur = vec![];
    dfs(0, &mut cur, &slots, &mut result);
    result.sort();
    result
}

fn main() {
    println!(
        "{:?}",
        generate_sentences(
            vec![vec!["happy".into(), "joy".into()], vec!["joy".into(), "cheerful".into()]],
            "I am happy today".into()
        )
    );
}

#[cfg(test)]
mod tests {
    use super::generate_sentences;

    #[test]
    fn example_one() {
        assert_eq!(
            generate_sentences(
                vec![vec!["happy".into(), "joy".into()], vec!["joy".into(), "cheerful".into()]],
                "I am happy today".into(),
            ),
            vec![
                "I am cheerful today".to_string(),
                "I am happy today".to_string(),
                "I am joyful today".to_string(),
            ]
        );
    }
}
