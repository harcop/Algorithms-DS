/// LeetCode #1202 - Smallest String With Swaps
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
            let p = self.find(self.parent[x]);
            self.parent[x] = p;
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

fn smallest_string_with_swaps(s: String, pairs: Vec<Vec<i32>>) -> String {
    let n = s.len();
    let mut uf = UnionFind::new(n);
    for p in pairs {
        uf.union(p[0] as usize, p[1] as usize);
    }
    let mut roots = vec![0usize; n];
    for i in 0..n {
        roots[i] = uf.find(i);
    }
    let mut groups: std::collections::HashMap<usize, Vec<char>> =
        std::collections::HashMap::new();
    for (i, c) in s.chars().enumerate() {
        groups.entry(roots[i]).or_default().push(c);
    }
    for v in groups.values_mut() {
        v.sort_unstable();
    }
    let mut ptr: std::collections::HashMap<usize, usize> = std::collections::HashMap::new();
    let mut out = vec![' '; n];
    for (i, c) in s.chars().enumerate() {
        let r = roots[i];
        let p = ptr.entry(r).or_insert(0);
        out[i] = groups.get(&r).unwrap()[*p];
        *p += 1;
    }
    out.into_iter().collect()
}

fn main() {
    println!(
        "{}",
        smallest_string_with_swaps("dcab".into(), vec![vec![0, 3], vec![1, 2]])
    );
}

#[cfg(test)]
mod tests {
    use super::smallest_string_with_swaps;

    #[test]
    fn example_one() {
        assert_eq!(
            smallest_string_with_swaps("dcab".into(), vec![vec![0, 3], vec![1, 2]]),
            "bacd"
        );
    }

    #[test]
    fn example_two() {
        assert_eq!(
            smallest_string_with_swaps("dcab".into(), vec![vec![0, 3], vec![1, 2], vec![0, 2]]),
            "abcd"
        );
    }
}
