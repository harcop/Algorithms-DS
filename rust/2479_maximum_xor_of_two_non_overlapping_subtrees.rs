/// LeetCode #2479 - Maximum XOR of Two Non-Overlapping Subtrees
struct Trie {
    children: [Option<Box<Trie>>; 2],
}

impl Trie {
    fn new() -> Self {
        Self {
            children: [None, None],
        }
    }

    fn insert(&mut self, value: i64) {
        let mut node = self;
        for bit in (0..48).rev() {
            let bit_value = ((value >> bit) & 1) as usize;
            if node.children[bit_value].is_none() {
                node.children[bit_value] = Some(Box::new(Trie::new()));
            }
            node = node.children[bit_value].as_mut().unwrap();
        }
    }

    fn search(&self, value: i64) -> i64 {
        let mut node = Some(self);
        let mut result = 0i64;
        for bit in (0..48).rev() {
            let Some(current) = node else {
                return result;
            };
            let bit_value = ((value >> bit) & 1) as usize;
            if current.children[bit_value ^ 1].is_some() {
                result = (result << 1) | 1;
                node = current.children[bit_value ^ 1].as_deref();
            } else {
                result <<= 1;
                node = current.children[bit_value].as_deref();
            }
        }
        result
    }
}

fn max_xor(n: i32, edges: Vec<Vec<i32>>, values: Vec<i32>) -> i64 {
    let n = n as usize;
    let mut graph = vec![Vec::new(); n];
    for edge in edges {
        let a = edge[0] as usize;
        let b = edge[1] as usize;
        graph[a].push(b);
        graph[b].push(a);
    }

    let mut subtree = vec![0i64; n];
    fn dfs_sum(
        node: usize,
        parent: usize,
        graph: &[Vec<usize>],
        values: &[i32],
        subtree: &mut [i64],
    ) -> i64 {
        let mut total = values[node] as i64;
        for &next in &graph[node] {
            if next != parent {
                total += dfs_sum(next, node, graph, values, subtree);
            }
        }
        subtree[node] = total;
        total
    }
    dfs_sum(0, usize::MAX, &graph, &values, &mut subtree);

    let mut answer = 0i64;
    let mut trie = Trie::new();
    fn dfs_xor(
        node: usize,
        parent: usize,
        graph: &[Vec<usize>],
        subtree: &[i64],
        trie: &mut Trie,
        answer: &mut i64,
    ) {
        *answer = (*answer).max(trie.search(subtree[node]));
        for &next in &graph[node] {
            if next != parent {
                dfs_xor(next, node, graph, subtree, trie, answer);
            }
        }
        trie.insert(subtree[node]);
    }
    dfs_xor(0, usize::MAX, &graph, &subtree, &mut trie, &mut answer);
    answer
}

fn main() {
    println!(
        "{}",
        max_xor(
            6,
            vec![vec![0, 1], vec![0, 2], vec![1, 3], vec![1, 4], vec![2, 5]],
            vec![2, 8, 3, 6, 2, 5]
        )
    );
}

#[cfg(test)]
mod tests {
    use super::max_xor;

    #[test]
    fn example_one() {
        assert_eq!(
            max_xor(
                6,
                vec![vec![0, 1], vec![0, 2], vec![1, 3], vec![1, 4], vec![2, 5]],
                vec![2, 8, 3, 6, 2, 5]
            ),
            24
        );
    }

    #[test]
    fn example_two() {
        assert_eq!(max_xor(3, vec![vec![0, 1], vec![1, 2]], vec![4, 6, 1]), 0);
    }
}
