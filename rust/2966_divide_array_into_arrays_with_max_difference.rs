/// LeetCode #2966 - Divide Array Into Arrays With Max Difference
fn divide_array(mut nums: Vec<i32>, k: i32) -> Vec<Vec<i32>> {
    nums.sort_unstable();
    let mut ans = Vec::new();
    let n = nums.len();
    for i in (0..n).step_by(3) {
        if nums[i + 2] - nums[i] > k {
            return vec![];
        }
        ans.push(vec![nums[i], nums[i + 1], nums[i + 2]]);
    }
    ans
}

fn main() {
    println!("{:?}", divide_array(vec![1, 3, 4, 8, 7, 9, 3, 5, 1], 2));
}

#[cfg(test)]
mod tests {
    use super::divide_array;

    #[test]
    fn example_one() {
        assert_eq!(
            divide_array(vec![1, 3, 4, 8, 7, 9, 3, 5, 1], 2),
            vec![vec![1, 1, 3], vec![3, 4, 5], vec![7, 8, 9]]
        );
    }

    #[test]
    fn example_two() {
        assert_eq!(divide_array(vec![2, 4, 2, 2, 5, 2], 2), Vec::<Vec<i32>>::new());
    }

    #[test]
    fn example_three() {
        assert_eq!(
            divide_array(
                vec![4, 2, 9, 8, 2, 12, 7, 12, 10, 5, 8, 5, 5, 7, 9, 2, 5, 11],
                14
            ),
            vec![
                vec![2, 2, 2],
                vec![4, 5, 5],
                vec![5, 5, 7],
                vec![7, 8, 8],
                vec![9, 9, 10],
                vec![11, 12, 12]
            ]
        );
    }
}
