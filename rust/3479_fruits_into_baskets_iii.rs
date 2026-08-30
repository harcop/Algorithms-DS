/// LeetCode #3479 - Fruits Into Baskets III
struct SegmentTree {
    nums: Vec<i32>,
    tr: Vec<i32>,
}

impl SegmentTree {
    fn new(nums: Vec<i32>) -> Self {
        let n = nums.len();
        let mut tree = Self {
            nums,
            tr: vec![0; n << 2],
        };
        tree.build(1, 1, n);
        tree
    }

    fn build(&mut self, u: usize, l: usize, r: usize) {
        if l == r {
            self.tr[u] = self.nums[l - 1];
            return;
        }
        let mid = (l + r) >> 1;
        self.build(u << 1, l, mid);
        self.build(u << 1 | 1, mid + 1, r);
        self.pushup(u);
    }

    fn modify(&mut self, u: usize, l: usize, r: usize, i: usize, v: i32) {
        if l == r {
            self.tr[u] = v;
            return;
        }
        let mid = (l + r) >> 1;
        if i <= mid {
            self.modify(u << 1, l, mid, i, v);
        } else {
            self.modify(u << 1 | 1, mid + 1, r, i, v);
        }
        self.pushup(u);
    }

    fn query(&self, u: usize, l: usize, r: usize, v: i32) -> i32 {
        if self.tr[u] < v {
            return -1;
        }
        if l == r {
            return l as i32;
        }
        let mid = (l + r) >> 1;
        if self.tr[u << 1] >= v {
            self.query(u << 1, l, mid, v)
        } else {
            self.query(u << 1 | 1, mid + 1, r, v)
        }
    }

    fn pushup(&mut self, u: usize) {
        self.tr[u] = self.tr[u << 1].max(self.tr[u << 1 | 1]);
    }
}

fn num_of_unplaced_fruits(fruits: Vec<i32>, baskets: Vec<i32>) -> i32 {
    let n = baskets.len();
    let mut tree = SegmentTree::new(baskets);
    let mut ans = 0;
    for x in fruits {
        let i = tree.query(1, 1, n, x);
        if i < 0 {
            ans += 1;
        } else {
            tree.modify(1, 1, n, i as usize, 0);
        }
    }
    ans
}

fn main() {
    println!("{}", num_of_unplaced_fruits(vec![4, 2, 5], vec![3, 5, 4]));
}

#[cfg(test)]
mod tests {
    use super::num_of_unplaced_fruits;

    #[test]
    fn example1() {
        assert_eq!(num_of_unplaced_fruits(vec![4, 2, 5], vec![3, 5, 4]), 1);
    }

    #[test]
    fn example2() {
        assert_eq!(num_of_unplaced_fruits(vec![3, 6, 1], vec![6, 4, 7]), 0);
    }
}
