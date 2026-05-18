/// LeetCode #1005 - Maximize Sum Of Array After K Negations
fn largest_sum_after_k_negations(nums: Vec<i32>, k: i32) -> i32 {
    let mut arr = nums;
    arr.sort_unstable();
    let mut k = k;
    for i in 0..arr.len() {
        if k == 0 {
            break;
        }
        if arr[i] < 0 {
            arr[i] = -arr[i];
            k -= 1;
        }
    }
    if k % 2 == 1 {
        arr.sort_unstable();
        arr[0] = -arr[0];
    }
    arr.iter().sum()
}

fn main() {
    println!("{}", largest_sum_after_k_negations(vec![4, 2, 3], 1));
}

#[cfg(test)]
mod tests {
    use super::largest_sum_after_k_negations;

    #[test]
    fn example_one() {
        assert_eq!(largest_sum_after_k_negations(vec![4, 2, 3], 1), 5);
    }

    #[test]
    fn example_two() {
        assert_eq!(largest_sum_after_k_negations(vec![3, -1, 0, 2], 3), 6);
    }
}
