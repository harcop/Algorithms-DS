/// LeetCode #1803 - Count Pairs With XOR in a Range
struct TrieNode {
    children: [Option<Box<TrieNode>>; 2],
    cnt: i32,
}

impl TrieNode {
    fn new() -> Self {
        TrieNode {
            children: [None, None],
            cnt: 0,
        }
    }
}

struct Trie {
    root: TrieNode,
}

impl Trie {
    fn new() -> Self {
        Trie {
            root: TrieNode::new(),
        }
    }

    fn insert(&mut self, x: i32) {
        let mut node = &mut self.root;
        for i in (0..=15).rev() {
            let v = ((x >> i) & 1) as usize;
            if node.children[v].is_none() {
                node.children[v] = Some(Box::new(TrieNode::new()));
            }
            node = node.children[v].as_mut().unwrap();
            node.cnt += 1;
        }
    }

    fn search(&self, x: i32, limit: i32) -> i32 {
        let mut node = Some(&self.root);
        let mut ans = 0i32;
        for i in (0..=15).rev() {
            let node_ref = match node {
                Some(n) => n,
                None => return ans,
            };
            let v = ((x >> i) & 1) as usize;
            if (limit >> i) & 1 == 1 {
                if let Some(ref child) = node_ref.children[v] {
                    ans += child.cnt;
                }
                node = node_ref.children[v ^ 1].as_deref();
            } else {
                node = node_ref.children[v].as_deref();
            }
        }
        ans
    }
}

fn count_pairs(nums: Vec<i32>, low: i32, high: i32) -> i32 {
    let mut tree = Trie::new();
    let mut ans = 0i32;
    for x in nums {
        ans += tree.search(x, high + 1) - tree.search(x, low);
        tree.insert(x);
    }
    ans
}

fn main() {
    println!("{}", count_pairs(vec![1, 4, 2, 7], 2, 6));
}

#[cfg(test)]
mod tests {
    use super::count_pairs;

    #[test]
    fn example_one() {
        assert_eq!(count_pairs(vec![1, 4, 2, 7], 2, 6), 6);
    }
}
