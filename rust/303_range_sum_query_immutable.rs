/// LeetCode #303 - Range Sum Query - Immutable
pub struct NumArray {
    pre: Vec<i32>,
}

impl NumArray {
    fn new(nums: Vec<i32>) -> Self {
        let mut pre = vec![0; nums.len() + 1];
        for i in 0..nums.len() {
            pre[i + 1] = pre[i] + nums[i];
        }
        NumArray { pre }
    }

    fn sum_range(&self, left: i32, right: i32) -> i32 {
        self.pre[right as usize + 1] - self.pre[left as usize]
    }
}

fn main() {
    let n = NumArray::new(vec![-2, 0, 3, -5, 2, -1]);
    println!("{}", n.sum_range(0, 2));
}

#[cfg(test)]
mod tests {
    use super::NumArray;

    #[test]
    fn example() {
        let n = NumArray::new(vec![-2, 0, 3, -5, 2, -1]);
        assert_eq!(n.sum_range(0, 2), 1);
        assert_eq!(n.sum_range(2, 5), -1);
        assert_eq!(n.sum_range(0, 5), -3);
    }
}
