/// LeetCode #2736 - Maximum Sum Queries
struct BinaryIndexedTree {
    n: usize,
    c: Vec<i32>,
}

impl BinaryIndexedTree {
    fn new(n: usize) -> Self {
        BinaryIndexedTree {
            n,
            c: vec![-1; n + 1],
        }
    }

    fn update(&mut self, mut x: usize, v: i32) {
        while x <= self.n {
            self.c[x] = self.c[x].max(v);
            x += x & x.wrapping_neg();
        }
    }

    fn query(&self, mut x: usize) -> i32 {
        let mut mx = -1;
        while x > 0 {
            mx = mx.max(self.c[x]);
            x -= x & x.wrapping_neg();
        }
        mx
    }
}

fn maximum_sum_queries(nums1: Vec<i32>, nums2: Vec<i32>, queries: Vec<Vec<i32>>) -> Vec<i32> {
    let n = nums1.len();
    let m = queries.len();
    let mut nums: Vec<(i32, i32)> = nums1.into_iter().zip(nums2).collect();
    nums.sort_by(|a, b| b.0.cmp(&a.0));
    let mut sorted2 = nums.iter().map(|(_, b)| *b).collect::<Vec<_>>();
    sorted2.sort_unstable();

    let mut idx: Vec<usize> = (0..m).collect();
    idx.sort_by(|&i, &j| queries[j][0].cmp(&queries[i][0]));

    let mut ans = vec![-1; m];
    let mut j = 0;
    let mut tree = BinaryIndexedTree::new(n);

    let search = |x: i32| -> usize {
        let mut l = 0;
        let mut r = n;
        while l < r {
            let mid = (l + r) / 2;
            if sorted2[mid] >= x {
                r = mid;
            } else {
                l = mid + 1;
            }
        }
        l
    };

    for &i in &idx {
        let x = queries[i][0];
        let y = queries[i][1];
        while j < n && nums[j].0 >= x {
            let k = n - search(nums[j].1);
            tree.update(k, nums[j].0 + nums[j].1);
            j += 1;
        }
        let k = n - search(y);
        ans[i] = tree.query(k);
    }
    ans
}

fn main() {
    println!(
        "{:?}",
        maximum_sum_queries(
            vec![4, 3, 1, 2],
            vec![2, 4, 9, 5],
            vec![vec![4, 1], vec![1, 3], vec![2, 5]]
        )
    );
}

#[cfg(test)]
mod tests {
    use super::maximum_sum_queries;

    #[test]
    fn example_one() {
        assert_eq!(
            maximum_sum_queries(
                vec![4, 3, 1, 2],
                vec![2, 4, 9, 5],
                vec![vec![4, 1], vec![1, 3], vec![2, 5]]
            ),
            vec![6, 10, 7]
        );
    }

    #[test]
    fn example_two() {
        assert_eq!(
            maximum_sum_queries(
                vec![3, 2, 5],
                vec![2, 3, 4],
                vec![vec![4, 4], vec![3, 2], vec![1, 1]]
            ),
            vec![9, 9, 9]
        );
    }

    #[test]
    fn example_three() {
        assert_eq!(
            maximum_sum_queries(vec![2, 1], vec![2, 3], vec![vec![3, 3]]),
            vec![-1]
        );
    }
}
