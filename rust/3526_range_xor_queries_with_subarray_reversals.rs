/// LeetCode #3526 - Range XOR Queries with Subarray Reversals
struct Node {
    val: i32,
    prio: u64,
    sz: usize,
    xor: i32,
    rev: bool,
    l: usize,
    r: usize,
}

struct Treap {
    nodes: Vec<Node>,
    seed: u64,
}

impl Treap {
    fn new() -> Self {
        Self {
            nodes: vec![Node {
                val: 0,
                prio: 0,
                sz: 0,
                xor: 0,
                rev: false,
                l: 0,
                r: 0,
            }],
            seed: 0x9e3779b97f4a7c15,
        }
    }

    fn next_prio(&mut self) -> u64 {
        self.seed = self.seed.wrapping_mul(6364136223846793005).wrapping_add(1);
        self.seed
    }

    fn alloc(&mut self, val: i32) -> usize {
        let prio = self.next_prio();
        self.nodes.push(Node {
            val,
            prio,
            sz: 1,
            xor: val,
            rev: false,
            l: 0,
            r: 0,
        });
        self.nodes.len() - 1
    }

    fn push(&mut self, t: usize) {
        if t == 0 || !self.nodes[t].rev {
            return;
        }
        let l = self.nodes[t].l;
        let r = self.nodes[t].r;
        self.nodes[t].l = r;
        self.nodes[t].r = l;
        self.nodes[t].rev = false;
        if l != 0 {
            self.nodes[l].rev ^= true;
        }
        if r != 0 {
            self.nodes[r].rev ^= true;
        }
    }

    fn pull(&mut self, t: usize) {
        let l = self.nodes[t].l;
        let r = self.nodes[t].r;
        let mut sz = 1;
        let mut xor = self.nodes[t].val;
        if l != 0 {
            sz += self.nodes[l].sz;
            xor ^= self.nodes[l].xor;
        }
        if r != 0 {
            sz += self.nodes[r].sz;
            xor ^= self.nodes[r].xor;
        }
        self.nodes[t].sz = sz;
        self.nodes[t].xor = xor;
    }

    fn split(&mut self, t: usize, k: usize) -> (usize, usize) {
        if t == 0 {
            return (0, 0);
        }
        self.push(t);
        let lsz = if self.nodes[t].l == 0 {
            0
        } else {
            self.nodes[self.nodes[t].l].sz
        };
        if k <= lsz {
            let left = self.nodes[t].l;
            let (a, b) = self.split(left, k);
            self.nodes[t].l = b;
            self.pull(t);
            (a, t)
        } else {
            let right = self.nodes[t].r;
            let (a, b) = self.split(right, k - lsz - 1);
            self.nodes[t].r = a;
            self.pull(t);
            (t, b)
        }
    }

    fn merge(&mut self, a: usize, b: usize) -> usize {
        if a == 0 {
            return b;
        }
        if b == 0 {
            return a;
        }
        self.push(a);
        self.push(b);
        if self.nodes[a].prio < self.nodes[b].prio {
            let r = self.nodes[a].r;
            let m = self.merge(r, b);
            self.nodes[a].r = m;
            self.pull(a);
            a
        } else {
            let l = self.nodes[b].l;
            let m = self.merge(a, l);
            self.nodes[b].l = m;
            self.pull(b);
            b
        }
    }
}

fn get_results(nums: Vec<i32>, queries: Vec<Vec<i32>>) -> Vec<i32> {
    let mut treap = Treap::new();
    let mut root = 0usize;
    for v in nums {
        let node = treap.alloc(v);
        root = treap.merge(root, node);
    }
    let mut ans = Vec::new();
    for q in queries {
        match q[0] {
            1 => {
                let idx = q[1] as usize;
                let (left, rest) = treap.split(root, idx);
                let (mid, right) = treap.split(rest, 1);
                treap.nodes[mid].val = q[2];
                treap.pull(mid);
                let merged = treap.merge(mid, right);
                root = treap.merge(left, merged);
            }
            2 => {
                let l = q[1] as usize;
                let r = q[2] as usize;
                let (left, rest) = treap.split(root, l);
                let (mid, right) = treap.split(rest, r - l + 1);
                ans.push(if mid == 0 { 0 } else { treap.nodes[mid].xor });
                let merged = treap.merge(mid, right);
                root = treap.merge(left, merged);
            }
            _ => {
                let l = q[1] as usize;
                let r = q[2] as usize;
                let (left, rest) = treap.split(root, l);
                let (mid, right) = treap.split(rest, r - l + 1);
                if mid != 0 {
                    treap.nodes[mid].rev ^= true;
                }
                let merged = treap.merge(mid, right);
                root = treap.merge(left, merged);
            }
        }
    }
    ans
}

fn main() {
    println!(
        "{:?}",
        get_results(vec![1, 2, 3, 4, 5], vec![vec![2, 1, 3], vec![1, 2, 10], vec![3, 0, 4], vec![2, 0, 4]])
    );
}

#[cfg(test)]
mod tests {
    use super::get_results;

    #[test]
    fn example1() {
        assert_eq!(
            get_results(
                vec![1, 2, 3, 4, 5],
                vec![vec![2, 1, 3], vec![1, 2, 10], vec![3, 0, 4], vec![2, 0, 4]]
            ),
            vec![5, 8]
        );
    }

    #[test]
    fn example2() {
        assert_eq!(
            get_results(vec![7, 8, 9], vec![vec![1, 0, 3], vec![2, 0, 2], vec![3, 1, 2]]),
            vec![2]
        );
    }
}
