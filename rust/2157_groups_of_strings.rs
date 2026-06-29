/// LeetCode #2157 - Groups of Strings
use std::collections::HashMap;

struct Dsu {
    parent: Vec<usize>,
    size: Vec<i32>,
}

impl Dsu {
    fn new(n: usize) -> Self {
        Dsu {
            parent: (0..n).collect(),
            size: vec![1; n],
        }
    }

    fn find(&mut self, x: usize) -> usize {
        if self.parent[x] != x {
            self.parent[x] = self.find(self.parent[x]);
        }
        self.parent[x]
    }

    fn union(&mut self, a: usize, b: usize) {
        let mut pa = self.find(a);
        let mut pb = self.find(b);
        if pa == pb {
            return;
        }
        if self.size[pa] < self.size[pb] {
            std::mem::swap(&mut pa, &mut pb);
        }
        self.parent[pb] = pa;
        self.size[pa] += self.size[pb];
    }
}

fn group_strings(words: Vec<String>) -> Vec<i32> {
    let n = words.len();
    let mut dsu = Dsu::new(n);
    let mut masks = Vec::with_capacity(n);
    let mut by_mask = HashMap::new();

    for (i, word) in words.iter().enumerate() {
        let mut mask = 0i32;
        for b in word.bytes() {
            mask |= 1 << (b - b'a');
        }
        if let Some(&j) = by_mask.get(&mask) {
            dsu.union(i, j);
        } else {
            by_mask.insert(mask, i);
        }
        masks.push(mask);
    }

    let mut by_deleted_mask = HashMap::new();
    for (i, &mask) in masks.iter().enumerate() {
        for bit in 0..26 {
            let next = mask ^ (1 << bit);
            if let Some(&j) = by_mask.get(&next) {
                dsu.union(i, j);
            }

            if (mask >> bit) & 1 == 1 {
                let deleted = mask ^ (1 << bit);
                if let Some(&j) = by_deleted_mask.get(&deleted) {
                    dsu.union(i, j);
                } else {
                    by_deleted_mask.insert(deleted, i);
                }
            }
        }
    }

    let mut groups = 0i32;
    let mut largest = 0i32;
    for i in 0..n {
        if dsu.find(i) == i {
            groups += 1;
            largest = largest.max(dsu.size[i]);
        }
    }
    vec![groups, largest]
}

fn main() {
    println!(
        "{:?}",
        group_strings(vec!["a".into(), "b".into(), "ab".into(), "cde".into()])
    );
}

#[cfg(test)]
mod tests {
    use super::group_strings;

    #[test]
    fn example_one() {
        assert_eq!(
            group_strings(vec!["a".into(), "b".into(), "ab".into(), "cde".into()]),
            vec![2, 3]
        );
    }

    #[test]
    fn example_two() {
        assert_eq!(
            group_strings(vec!["a".into(), "ab".into(), "abc".into()]),
            vec![1, 3]
        );
    }
}
