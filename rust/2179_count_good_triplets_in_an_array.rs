/// LeetCode #2179 - Count Good Triplets in an Array
struct Fenwick {
    tree: Vec<i64>,
}

impl Fenwick {
    fn new(n: usize) -> Self {
        Fenwick {
            tree: vec![0; n + 1],
        }
    }

    fn add(&mut self, mut i: usize, delta: i64) {
        i += 1;
        while i < self.tree.len() {
            self.tree[i] += delta;
            i += i & (!i + 1);
        }
    }

    fn sum(&self, mut i: usize) -> i64 {
        let mut ans = 0i64;
        while i > 0 {
            ans += self.tree[i];
            i &= i - 1;
        }
        ans
    }
}

fn good_triplets(nums1: Vec<i32>, nums2: Vec<i32>) -> i64 {
    let n = nums1.len();
    let mut pos = vec![0usize; n];
    for (i, &x) in nums2.iter().enumerate() {
        pos[x as usize] = i;
    }
    let arr: Vec<usize> = nums1.iter().map(|&x| pos[x as usize]).collect();

    let mut left = vec![0i64; n];
    let mut bit = Fenwick::new(n);
    for i in 0..n {
        left[i] = bit.sum(arr[i]);
        bit.add(arr[i], 1);
    }

    let mut ans = 0i64;
    let mut bit = Fenwick::new(n);
    for i in (0..n).rev() {
        let smaller_or_equal = bit.sum(arr[i] + 1);
        let right_greater = (n - i - 1) as i64 - smaller_or_equal;
        ans += left[i] * right_greater;
        bit.add(arr[i], 1);
    }
    ans
}

fn main() {
    println!("{}", good_triplets(vec![2, 0, 1, 3], vec![0, 1, 2, 3]));
}

#[cfg(test)]
mod tests {
    use super::good_triplets;

    #[test]
    fn example_one() {
        assert_eq!(good_triplets(vec![2, 0, 1, 3], vec![0, 1, 2, 3]), 1);
    }

    #[test]
    fn example_two() {
        assert_eq!(good_triplets(vec![4, 0, 1, 3, 2], vec![4, 1, 0, 2, 3]), 4);
    }
}
