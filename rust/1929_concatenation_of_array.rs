/// LeetCode #1929 - Concatenation of Array
fn get_concatenation(nums: Vec<i32>) -> Vec<i32> {
    let mut ans = nums.clone();
    ans.extend_from_slice(&nums);
    ans
}

fn main() {
    println!("{:?}", get_concatenation(vec![1, 2, 1]));
}

#[cfg(test)]
mod tests {
    use super::get_concatenation;

    #[test]
    fn example_one() {
        assert_eq!(get_concatenation(vec![1, 2, 1]), vec![1, 2, 1, 1, 2, 1]);
    }
}
