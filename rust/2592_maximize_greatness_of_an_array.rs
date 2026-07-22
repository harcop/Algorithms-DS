/// LeetCode #2592 - Maximize Greatness of an Array
fn maximize_greatness(mut nums: Vec<i32>) -> i32 {
    nums.sort_unstable();
    let mut i = 0usize;
    for &x in &nums {
        if x > nums[i] {
            i += 1;
        }
    }
    i as i32
}

fn main() {
    println!("{}", maximize_greatness(vec![1, 3, 5, 2, 1, 3, 1]));
}

#[cfg(test)]
mod tests {
    use super::maximize_greatness;

    #[test]
    fn example_one() {
        assert_eq!(maximize_greatness(vec![1, 3, 5, 2, 1, 3, 1]), 4);
    }

    #[test]
    fn example_two() {
        assert_eq!(maximize_greatness(vec![1, 2, 3, 4]), 3);
    }
}
