/// LeetCode #2089 - Find Target Indices After Sorting Array
fn target_indices(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let less = nums.iter().filter(|&&num| num < target).count() as i32;
    let equal = nums.iter().filter(|&&num| num == target).count() as i32;
    (less..less + equal).collect()
}

fn main() {
    println!("{:?}", target_indices(vec![1, 2, 5, 2, 3], 2));
}

#[cfg(test)]
mod tests {
    use super::target_indices;

    #[test]
    fn example_one() {
        assert_eq!(target_indices(vec![1, 2, 5, 2, 3], 2), vec![1, 2]);
    }

    #[test]
    fn example_two() {
        assert_eq!(target_indices(vec![1, 2, 5, 2, 3], 3), vec![3]);
    }

    #[test]
    fn example_three() {
        assert_eq!(target_indices(vec![1, 2, 5, 2, 3], 5), vec![4]);
    }
}
