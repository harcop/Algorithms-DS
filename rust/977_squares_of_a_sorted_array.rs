/// LeetCode #977 - Squares of a Sorted Array
fn sorted_squares(nums: Vec<i32>) -> Vec<i32> {
    let mut l = 0usize;
    let mut r = nums.len();
    let mut out = vec![0; nums.len()];
    for i in (0..nums.len()).rev() {
        let (a, b) = (nums[l] * nums[l], nums[r - 1] * nums[r - 1]);
        if a > b {
            out[i] = a;
            l += 1;
        } else {
            out[i] = b;
            r -= 1;
        }
    }
    out
}

fn main() {
    println!("{:?}", sorted_squares(vec![-4, -1, 0, 3, 10]));
}

#[cfg(test)]
mod tests {
    use super::sorted_squares;

    #[test]
    fn example_one() {
        assert_eq!(sorted_squares(vec![-4, -1, 0, 3, 10]), vec![0, 1, 9, 16, 100]);
    }

    #[test]
    fn example_two() {
        assert_eq!(sorted_squares(vec![-7, -3, 2, 3, 11]), vec![4, 9, 9, 49, 121]);
    }
}
