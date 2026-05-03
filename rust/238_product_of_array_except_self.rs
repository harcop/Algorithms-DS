/// LeetCode #238 - Product of Array Except Self
fn product_except_self(nums: Vec<i32>) -> Vec<i32> {
    let n = nums.len();
    let mut out = vec![1; n];
    let mut p = 1;
    for i in 0..n {
        out[i] = p;
        p *= nums[i];
    }
    p = 1;
    for i in (0..n).rev() {
        out[i] *= p;
        p *= nums[i];
    }
    out
}

fn main() {
    println!("{:?}", product_except_self(vec![1, 2, 3, 4]));
}

#[cfg(test)]
mod tests {
    use super::product_except_self;

    #[test]
    fn example_one() {
        assert_eq!(product_except_self(vec![1, 2, 3, 4]), vec![24, 12, 8, 6]);
    }

    #[test]
    fn example_two() {
        assert_eq!(product_except_self(vec![-1, 1, 0, -3, 3]), vec![0, 0, 9, 0, 0]);
    }
}
