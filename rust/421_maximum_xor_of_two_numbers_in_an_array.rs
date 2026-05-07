/// LeetCode #421 - Maximum XOR of Two Numbers in an Array
struct Trie {
    ch: [Option<Box<Trie>>; 2],
}

impl Trie {
    fn new() -> Self {
        Self { ch: [None, None] }
    }

    fn insert(&mut self, x: i32) {
        let mut cur = self;
        for k in (0..32).rev() {
            let b = ((x >> k) & 1) as usize;
            if cur.ch[b].is_none() {
                cur.ch[b] = Some(Box::new(Trie::new()));
            }
            cur = cur.ch[b].as_mut().unwrap();
        }
    }

    fn best_xor(&self, x: i32) -> i32 {
        let mut cur = self;
        let mut ans = 0i32;
        for k in (0..32).rev() {
            let b = ((x >> k) & 1) as usize;
            let want = 1 - b;
            if cur.ch[want].is_some() {
                ans |= 1 << k;
                cur = cur.ch[want].as_ref().unwrap();
            } else {
                cur = cur.ch[b].as_ref().unwrap();
            }
        }
        ans
    }
}

fn find_maximum_xor(nums: Vec<i32>) -> i32 {
    let mut t = Trie::new();
    for &x in &nums {
        t.insert(x);
    }
    let mut best = 0;
    for x in nums {
        best = best.max(t.best_xor(x));
    }
    best
}

fn main() {
    println!("{}", find_maximum_xor(vec![3, 10, 5, 25, 2, 8]));
}

#[cfg(test)]
mod tests {
    use super::find_maximum_xor;

    #[test]
    fn example_one() {
        assert_eq!(find_maximum_xor(vec![3, 10, 5, 25, 2, 8]), 28);
    }
}
