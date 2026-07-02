/// LeetCode #2213 - Longest Substring of One Repeating Character
#[derive(Clone, Copy)]
struct Node {
    l: usize,
    r: usize,
    lmx: i32,
    rmx: i32,
    mx: i32,
}

struct SegmentTree {
    s: Vec<u8>,
    tr: Vec<Node>,
}

impl SegmentTree {
    fn new(s: &str) -> Self {
        let s: Vec<u8> = s.bytes().collect();
        let n = s.len();
        let mut tree = Self {
            s,
            tr: vec![
                Node {
                    l: 0,
                    r: 0,
                    lmx: 0,
                    rmx: 0,
                    mx: 0,
                };
                n * 4
            ],
        };
        tree.build(1, 1, n);
        tree
    }

    fn build(&mut self, u: usize, l: usize, r: usize) {
        self.tr[u] = Node {
            l,
            r,
            lmx: 1,
            rmx: 1,
            mx: 1,
        };
        if l == r {
            return;
        }
        let mid = (l + r) / 2;
        self.build(u << 1, l, mid);
        self.build(u << 1 | 1, mid + 1, r);
        self.pushup(u);
    }

    fn pushup(&mut self, u: usize) {
        let left = self.tr[u << 1];
        let right = self.tr[u << 1 | 1];
        let mut root = Node {
            l: left.l,
            r: right.r,
            lmx: left.lmx,
            rmx: right.rmx,
            mx: left.mx.max(right.mx),
        };

        let a = (left.r - left.l + 1) as i32;
        let b = (right.r - right.l + 1) as i32;
        if self.s[left.r - 1] == self.s[right.l - 1] {
            if left.lmx == a {
                root.lmx += right.lmx;
            }
            if right.rmx == b {
                root.rmx += left.rmx;
            }
            root.mx = root.mx.max(left.rmx + right.lmx);
        }

        self.tr[u] = root;
    }

    fn modify(&mut self, u: usize, x: usize, v: u8) {
        if self.tr[u].l == self.tr[u].r {
            self.s[x - 1] = v;
            return;
        }
        let mid = (self.tr[u].l + self.tr[u].r) / 2;
        if x <= mid {
            self.modify(u << 1, x, v);
        } else {
            self.modify(u << 1 | 1, x, v);
        }
        self.pushup(u);
    }

    fn max_length(&self) -> i32 {
        self.tr[1].mx
    }
}

fn longest_repeating(s: String, query_characters: String, query_indices: Vec<i32>) -> Vec<i32> {
    let mut tree = SegmentTree::new(&s);
    let mut ans = Vec::new();
    for (&idx, ch) in query_indices.iter().zip(query_characters.bytes()) {
        tree.modify(1, (idx + 1) as usize, ch);
        ans.push(tree.max_length());
    }
    ans
}

fn main() {
    println!(
        "{:?}",
        longest_repeating("babacc".into(), "bcb".into(), vec![1, 3, 3])
    );
}

#[cfg(test)]
mod tests {
    use super::longest_repeating;

    #[test]
    fn example_one() {
        assert_eq!(
            longest_repeating("babacc".into(), "bcb".into(), vec![1, 3, 3]),
            vec![3, 3, 4]
        );
    }

    #[test]
    fn example_two() {
        assert_eq!(
            longest_repeating("abyzz".into(), "aa".into(), vec![2, 1]),
            vec![2, 3]
        );
    }
}
