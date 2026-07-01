/// LeetCode #2200 - Find All K-Distant Indices in an Array
fn find_k_distant_indices(nums: Vec<i32>, key: i32, k: i32) -> Vec<i32> {
    let n = nums.len();
    let mut ans = Vec::new();
    for i in 0..n {
        for j in 0..n {
            if (i as i32 - j as i32).abs() <= k && nums[j] == key {
                ans.push(i as i32);
                break;
            }
        }
    }
    ans
}

fn main() {
    println!(
        "{:?}",
        find_k_distant_indices(vec![3, 4, 9, 1, 3, 9, 5], 9, 1)
    );
}

#[cfg(test)]
mod tests {
    use super::find_k_distant_indices;

    #[test]
    fn example_one() {
        assert_eq!(
            find_k_distant_indices(vec![3, 4, 9, 1, 3, 9, 5], 9, 1),
            vec![1, 2, 3, 4, 5, 6]
        );
    }

    #[test]
    fn example_two() {
        assert_eq!(
            find_k_distant_indices(vec![2, 2, 2, 2, 2], 2, 2),
            vec![0, 1, 2, 3, 4]
        );
    }
}
