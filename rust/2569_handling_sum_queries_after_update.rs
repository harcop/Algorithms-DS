/// LeetCode #2569 - Handling Sum Queries After Update
struct LazySegmentTree {
    tree: Vec<i64>,
    lazy: Vec<bool>,
    n: usize,
}

impl LazySegmentTree {
    fn new(nums: &[i32]) -> Self {
        let n = nums.len();
        let mut st = Self {
            tree: vec![0; 4 * n],
            lazy: vec![false; 4 * n],
            n,
        };
        st.build(nums, 0, 0, n - 1);
        st
    }

    fn build(&mut self, nums: &[i32], i: usize, start: usize, end: usize) {
        if start == end {
            self.tree[i] = nums[start] as i64;
            return;
        }
        let mid = (start + end) / 2;
        self.build(nums, 2 * i + 1, start, mid);
        self.build(nums, 2 * i + 2, mid + 1, end);
        self.tree[i] = self.tree[2 * i + 1] + self.tree[2 * i + 2];
    }

    fn flip(&mut self, i: usize, start: usize, end: usize) {
        self.tree[i] = (end - start + 1) as i64 - self.tree[i];
        if start < end {
            self.lazy[2 * i + 1] = !self.lazy[2 * i + 1];
            self.lazy[2 * i + 2] = !self.lazy[2 * i + 2];
        }
    }

    fn propagate(&mut self, i: usize, start: usize, end: usize) {
        if self.lazy[i] {
            self.flip(i, start, end);
            self.lazy[i] = false;
        }
    }

    fn update_range(&mut self, i: usize, start: usize, end: usize, l: usize, r: usize) {
        self.propagate(i, start, end);
        if start > r || end < l {
            return;
        }
        if start >= l && end <= r {
            self.flip(i, start, end);
            return;
        }
        let mid = (start + end) / 2;
        self.update_range(2 * i + 1, start, mid, l, r);
        self.update_range(2 * i + 2, mid + 1, end, l, r);
        self.tree[i] = self.tree[2 * i + 1] + self.tree[2 * i + 2];
    }

    fn get_sum(&self) -> i64 {
        self.tree[0]
    }
}

fn handle_query(nums1: Vec<i32>, nums2: Vec<i32>, queries: Vec<Vec<i32>>) -> Vec<i64> {
    let mut tree = LazySegmentTree::new(&nums1);
    let mut sum_nums2: i64 = nums2.iter().map(|&x| x as i64).sum();
    let mut ans = Vec::new();

    for q in queries {
        let typ = q[0];
        let l = q[1];
        let r = q[2];
        if typ == 1 {
            tree.update_range(0, 0, tree.n - 1, l as usize, r as usize);
        } else if typ == 2 {
            sum_nums2 += l as i64 * tree.get_sum();
        } else {
            ans.push(sum_nums2);
        }
    }
    ans
}

fn main() {
    println!(
        "{:?}",
        handle_query(
            vec![1, 0, 1],
            vec![0, 0, 0],
            vec![vec![1, 1, 1], vec![2, 1, 0], vec![3, 0, 0]]
        )
    );
}

#[cfg(test)]
mod tests {
    use super::handle_query;

    #[test]
    fn example_one() {
        assert_eq!(
            handle_query(
                vec![1, 0, 1],
                vec![0, 0, 0],
                vec![vec![1, 1, 1], vec![2, 1, 0], vec![3, 0, 0]]
            ),
            vec![3]
        );
    }

    #[test]
    fn example_two() {
        assert_eq!(
            handle_query(vec![1], vec![5], vec![vec![2, 0, 0], vec![3, 0, 0]]),
            vec![5]
        );
    }
}
