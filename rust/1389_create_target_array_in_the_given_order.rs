/// LeetCode #1389 - Create Target Array In The Given Order
fn create_target_array(nums: Vec<i32>, index: Vec<i32>) -> Vec<i32> {
    let mut ans = Vec::new();
    for (num, idx) in nums.into_iter().zip(index) {
        ans.insert(idx as usize, num);
    }
    ans
}

fn main() {
    println!("{:?}", create_target_array(vec![0, 1, 2, 3, 4], vec![0, 1, 2, 2, 1]));
}

#[cfg(test)]
mod tests {
    use super::create_target_array;

    #[test]
    fn example_one() {
        assert_eq!(
            create_target_array(vec![0, 1, 2, 3, 4], vec![0, 1, 2, 2, 1]),
            vec![0, 4, 1, 3, 2]
        );
    }

    #[test]
    fn example_two() {
        assert_eq!(create_target_array(vec![1, 2, 3, 4, 0], vec![0, 1, 2, 3, 0]), vec![0, 1, 2, 3, 4]);
    }
}

