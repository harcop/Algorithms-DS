/// LeetCode #3072 - Distribute Elements Into Two Arrays II
struct Fenwick {
    tree: Vec<i32>,
}

impl Fenwick {
    fn new(n: usize) -> Self {
        Self { tree: vec![0; n + 1] }
    }

    fn add(&mut self, mut i: usize, delta: i32) {
        i += 1;
        while i < self.tree.len() {
            self.tree[i] += delta;
            i += i & i.wrapping_neg();
        }
    }

    fn prefix(&self, mut i: usize) -> i32 {
        i += 1;
        let mut sum = 0;
        while i > 0 {
            sum += self.tree[i];
            i -= i & i.wrapping_neg();
        }
        sum
    }

    fn total(&self) -> i32 {
        self.prefix(self.tree.len() - 2)
    }

    fn greater_count(&self, idx: usize) -> i32 {
        self.total() - self.prefix(idx)
    }
}

fn result_array_ii(nums: Vec<i32>) -> Vec<i32> {
    let mut vals: Vec<i32> = nums.iter().copied().collect();
    vals.sort_unstable();
    vals.dedup();
    let rank = |x: i32| vals.binary_search(&x).unwrap();

    let mut bit1 = Fenwick::new(vals.len());
    let mut bit2 = Fenwick::new(vals.len());
    bit1.add(rank(nums[0]), 1);
    bit2.add(rank(nums[1]), 1);

    let mut arr1 = vec![nums[0]];
    let mut arr2 = vec![nums[1]];

    for &x in &nums[2..] {
        let r = rank(x);
        let gc1 = bit1.greater_count(r);
        let gc2 = bit2.greater_count(r);

        if gc1 > gc2 || (gc1 == gc2 && arr1.len() <= arr2.len()) {
            arr1.push(x);
            bit1.add(r, 1);
        } else {
            arr2.push(x);
            bit2.add(r, 1);
        }
    }

    arr1.extend(arr2);
    arr1
}

fn main() {
    println!("{:?}", result_array_ii(vec![2, 1, 3, 3]));
}

#[cfg(test)]
mod tests {
    use super::result_array_ii;

    #[test]
    fn example1() {
        assert_eq!(result_array_ii(vec![2, 1, 3, 3]), vec![2, 3, 1, 3]);
    }

    #[test]
    fn example2() {
        assert_eq!(result_array_ii(vec![5, 14, 3, 1, 2]), vec![5, 3, 1, 2, 14]);
    }

    #[test]
    fn example3() {
        assert_eq!(result_array_ii(vec![3, 3, 3, 3]), vec![3, 3, 3, 3]);
    }
}
