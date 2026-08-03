/// LeetCode #2935 - Maximum Strong Pair XOR II
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

    fn insert(&mut self, x: i32) {
        let mut node = self;
        for i in (0..=20).rev() {
            let v = ((x >> i) & 1) as usize;
            if node.children[v].is_none() {
                node.children[v] = Some(Box::new(Trie::new()));
            }
            node = node.children[v].as_mut().unwrap();
            node.cnt += 1;
        }
    }

    fn remove(&mut self, x: i32) {
        let mut node = self;
        for i in (0..=20).rev() {
            let v = ((x >> i) & 1) as usize;
            node = node.children[v].as_mut().unwrap();
            node.cnt -= 1;
        }
    }

    fn search(&self, x: i32) -> i32 {
        let mut node = self;
        let mut ans = 0;
        for i in (0..=20).rev() {
            let v = ((x >> i) & 1) as usize;
            let alt = v ^ 1;
            if node.children[alt].as_ref().map(|c| c.cnt > 0).unwrap_or(false) {
                ans |= 1 << i;
                node = node.children[alt].as_ref().unwrap();
            } else {
                node = node.children[v].as_ref().unwrap();
            }
        }
        ans
    }
}

fn maximum_strong_pair_xor(mut nums: Vec<i32>) -> i32 {
    nums.sort_unstable();
    let mut tree = Trie::new();
    let mut ans = 0;
    let mut i = 0usize;
    for &y in &nums {
        tree.insert(y);
        while y > nums[i] * 2 {
            tree.remove(nums[i]);
            i += 1;
        }
        ans = ans.max(tree.search(y));
    }
    ans
}

fn main() {
    println!("{}", maximum_strong_pair_xor(vec![1, 2, 3, 4, 5]));
}

#[cfg(test)]
mod tests {
    use super::maximum_strong_pair_xor;

    #[test]
    fn example_one() {
        assert_eq!(maximum_strong_pair_xor(vec![1, 2, 3, 4, 5]), 7);
    }

    #[test]
    fn example_two() {
        assert_eq!(maximum_strong_pair_xor(vec![10, 100]), 0);
    }

    #[test]
    fn example_three() {
        assert_eq!(maximum_strong_pair_xor(vec![500, 520, 2500, 3000]), 1020);
    }
}
