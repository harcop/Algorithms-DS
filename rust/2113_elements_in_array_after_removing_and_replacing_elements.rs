/// LeetCode #2113 - Elements in Array After Removing and Replacing Elements
fn element_in_nums(nums: Vec<i32>, queries: Vec<Vec<i32>>) -> Vec<i32> {
    let n = nums.len() as i32;
    queries
        .into_iter()
        .map(|q| {
            let t = q[0] % (2 * n);
            let i = q[1];

            if t < n && i < n - t {
                nums[(i + t) as usize]
            } else if t > n && i < t - n {
                nums[i as usize]
            } else {
                -1
            }
        })
        .collect()
}

fn main() {
    println!(
        "{:?}",
        element_in_nums(vec![0, 1, 2], vec![vec![0, 2], vec![2, 0]])
    );
}

#[cfg(test)]
mod tests {
    use super::element_in_nums;

    #[test]
    fn example_one() {
        assert_eq!(
            element_in_nums(
                vec![0, 1, 2],
                vec![vec![0, 2], vec![2, 0], vec![3, 2], vec![5, 0]]
            ),
            vec![2, 2, -1, 0]
        );
    }

    #[test]
    fn example_two() {
        assert_eq!(
            element_in_nums(
                vec![2],
                vec![vec![0, 0], vec![1, 0], vec![2, 0], vec![3, 0]]
            ),
            vec![2, -1, 2, -1]
        );
    }
}
