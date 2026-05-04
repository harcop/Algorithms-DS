/// LeetCode #307 - Range Sum Query - Mutable (rebuild prefix; O(n) update for clarity)
pub struct NumArray {
    nums: Vec<i32>,
}

impl NumArray {
    fn new(nums: Vec<i32>) -> Self {
        NumArray { nums }
    }

    fn update(&mut self, index: i32, val: i32) {
        self.nums[index as usize] = val;
    }

    fn sum_range(&self, left: i32, right: i32) -> i32 {
        self.nums[left as usize..=right as usize].iter().sum()
    }
}

fn main() {
    let n = NumArray::new(vec![1, 3, 5]);
    println!("{}", n.sum_range(0, 2));
}

#[cfg(test)]
mod tests {
    use super::NumArray;

    #[test]
    fn example() {
        let mut n = NumArray::new(vec![1, 3, 5]);
        assert_eq!(n.sum_range(0, 2), 9);
        n.update(1, 2);
        assert_eq!(n.sum_range(0, 2), 8);
    }
}
