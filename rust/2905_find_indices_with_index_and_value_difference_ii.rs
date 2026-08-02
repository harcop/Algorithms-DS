/// LeetCode #2905 - Find Indices With Index and Value Difference II
fn find_indices(nums: Vec<i32>, index_difference: i32, value_difference: i32) -> Vec<i32> {
    let index_difference = index_difference as usize;
    let mut mi = 0;
    let mut mx = 0;
    for i in index_difference..nums.len() {
        let j = i - index_difference;
        if nums[j] < nums[mi] {
            mi = j;
        }
        if nums[j] > nums[mx] {
            mx = j;
        }
        if nums[i] - nums[mi] >= value_difference {
            return vec![mi as i32, i as i32];
        }
        if nums[mx] - nums[i] >= value_difference {
            return vec![mx as i32, i as i32];
        }
    }
    vec![-1, -1]
}

fn main() {
    println!("{:?}", find_indices(vec![5, 1, 4, 1], 2, 4));
}

#[cfg(test)]
mod tests {
    use super::find_indices;

    #[test]
    fn example_one() {
        let ans = find_indices(vec![5, 1, 4, 1], 2, 4);
        assert!(ans == vec![0, 3] || ans == vec![3, 0]);
    }

    #[test]
    fn example_two() {
        let ans = find_indices(vec![2, 1], 0, 0);
        assert_eq!(ans.len(), 2);
        assert!(ans[0] >= 0 && ans[1] >= 0);
    }

    #[test]
    fn example_three() {
        assert_eq!(find_indices(vec![1, 2, 3], 2, 4), vec![-1, -1]);
    }
}
