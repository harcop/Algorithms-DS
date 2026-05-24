/// LeetCode #1308 - Smallest String With Swaps
struct Dsu {
    parent: Vec<usize>,
}

impl Dsu {
    fn new(n: usize) -> Self {
        Self { parent: (0..n).collect() }
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

fn smallest_string_with_swaps(s: String, pairs: Vec<Vec<i32>>) -> String {
    let n = s.len();
    let mut dsu = Dsu::new(n);
    for p in pairs {
        dsu.union(p[0] as usize, p[1] as usize);
    }
    let mut groups: std::collections::HashMap<usize, Vec<char>> = std::collections::HashMap::new();
    for (i, c) in s.chars().enumerate() {
        groups.entry(dsu.find(i)).or_default().push(c);
    }
    for v in groups.values_mut() {
        v.sort_by(|a, b| b.cmp(a));
    }
    let mut ans = vec![' '; n];
    for i in 0..n {
        let g = dsu.find(i);
        let ch = groups.get_mut(&g).unwrap().pop().unwrap();
        ans[i] = ch;
    }
    ans.into_iter().collect()
}

fn main() {
    println!("{}", smallest_string_with_swaps("dcab".to_string(), vec![vec![0, 3], vec![1, 2]]));
}

#[cfg(test)]
mod tests {
    use super::smallest_string_with_swaps;

    #[test]
    fn example_one() {
        assert_eq!(smallest_string_with_swaps("dcab".to_string(), vec![vec![0, 3], vec![1, 2]]), "bacd");
    }

    #[test]
    fn example_two() {
        assert_eq!(smallest_string_with_swaps("dcab".to_string(), vec![vec![0, 3], vec![1, 2], vec![0, 2]]), "abcd");
    }
}
