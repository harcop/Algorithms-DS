/// LeetCode #244 - Shortest Word Distance II
use std::collections::HashMap;

pub struct WordDistance {
    idx: HashMap<String, Vec<usize>>,
}

impl WordDistance {
    fn new(words_dict: Vec<String>) -> Self {
        let mut idx: HashMap<String, Vec<usize>> = HashMap::new();
        for (i, w) in words_dict.into_iter().enumerate() {
            idx.entry(w).or_default().push(i);
        }
        WordDistance { idx }
    }

    fn shortest(&self, word1: String, word2: String) -> i32 {
        if word1 == word2 {
            let v = &self.idx[&word1];
            if v.len() < 2 {
                return 0;
            }
            let mut d = usize::MAX;
            for w in v.windows(2) {
                d = d.min(w[1] - w[0]);
            }
            return d as i32;
        }
        let a = &self.idx[&word1];
        let b = &self.idx[&word2];
        let mut i = 0usize;
        let mut j = 0usize;
        let mut best = usize::MAX;
        while i < a.len() && j < b.len() {
            best = best.min(a[i].abs_diff(b[j]));
            if a[i] < b[j] {
                i += 1;
            } else {
                j += 1;
            }
        }
        best as i32
    }
}

fn main() {
    let wd = WordDistance::new(vec!["a".into(), "b".into()]);
    println!("{}", wd.shortest("a".into(), "b".into()));
}

#[cfg(test)]
mod tests {
    use super::WordDistance;

    #[test]
    fn example() {
        let wd = WordDistance::new(vec![
            "practice".into(),
            "makes".into(),
            "perfect".into(),
            "coding".into(),
            "makes".into(),
        ]);
        assert_eq!(wd.shortest("makes".into(), "coding".into()), 1);
        assert_eq!(wd.shortest("makes".into(), "makes".into()), 3);
    }
}
