/// LeetCode #1938 - Maximum Genetic Difference Query
use std::collections::HashMap;

struct Trie {
    children: [Option<Box<Trie>>; 2],
    cnt: i32,
}

impl Trie {
    fn new() -> Self {
        Self {
            children: [None, None],
            cnt: 0,
        }
    }

    fn insert(&mut self, num: i32, delta: i32) {
        let mut node = self;
        for b in (0..=17).rev() {
            let bit = ((num >> b) & 1) as usize;
            if node.children[bit].is_none() {
                node.children[bit] = Some(Box::new(Trie::new()));
            }
            node = node.children[bit].as_mut().unwrap();
            node.cnt += delta;
        }
    }

    fn query(&self, num: i32) -> i32 {
        let mut node = self;
        let mut ans = 0;
        for b in (0..=17).rev() {
            let bit = ((num >> b) & 1) as usize;
            let want = 1 - bit;
            if let Some(next) = node.children[want].as_ref() {
                if next.cnt > 0 {
                    ans |= 1 << b;
                    node = next;
                    continue;
                }
            }
            node = node.children[bit].as_ref().unwrap();
        }
        ans
    }
}

fn max_genetic_difference(parents: Vec<i32>, queries: Vec<Vec<i32>>) -> Vec<i32> {
    let n = parents.len();
    let mut children = vec![vec![]; n];
    let mut root = 0usize;
    for i in 0..n {
        if parents[i] == -1 {
            root = i;
        } else {
            children[parents[i] as usize].push(i);
        }
    }

    let mut by_node: HashMap<usize, Vec<(i32, usize)>> = HashMap::new();
    for (idx, q) in queries.iter().enumerate() {
        by_node
            .entry(q[0] as usize)
            .or_default()
            .push((q[1], idx));
    }

    let mut ans = vec![0; queries.len()];
    let mut trie = Trie::new();

    fn dfs(
        u: usize,
        children: &[Vec<usize>],
        by_node: &HashMap<usize, Vec<(i32, usize)>>,
        trie: &mut Trie,
        ans: &mut [i32],
    ) {
        trie.insert(u as i32, 1);
        if let Some(qs) = by_node.get(&u) {
            for &(val, idx) in qs {
                ans[idx] = trie.query(val);
            }
        }
        for &v in &children[u] {
            dfs(v, children, by_node, trie, ans);
        }
        trie.insert(u as i32, -1);
    }

    dfs(root, &children, &by_node, &mut trie, &mut ans);
    ans
}

fn main() {
    println!(
        "{:?}",
        max_genetic_difference(
            vec![-1, 0, 1, 1],
            vec![vec![0, 2], vec![3, 2], vec![2, 5]]
        )
    );
}

#[cfg(test)]
mod tests {
    use super::max_genetic_difference;

    #[test]
    fn example_one() {
        assert_eq!(
            max_genetic_difference(
                vec![-1, 0, 1, 1],
                vec![vec![0, 2], vec![3, 2], vec![2, 5]]
            ),
            vec![2, 3, 7]
        );
    }

    #[test]
    fn example_two() {
        assert_eq!(
            max_genetic_difference(
                vec![3, 7, -1, 2, 0, 7, 0, 2],
                vec![vec![4, 6], vec![1, 15], vec![0, 5]]
            ),
            vec![6, 14, 7]
        );
    }
}
