/// LeetCode #2717 - Semi-Ordered Permutation
fn semi_ordered_permutation(nums: Vec<i32>) -> i32 {
    let n = nums.len();
    let mut i = 0;
    let mut j = 0;
    for k in 0..n {
        if nums[k] == 1 {
            i = k;
        }
        if nums[k] == n as i32 {
            j = k;
        }
    }
    let k = if i < j { 1 } else { 2 };
    (i + n - j - k) as i32
}

fn main() {
    println!("{}", semi_ordered_permutation(vec![2, 1, 4, 3]));
}

#[cfg(test)]
mod tests {
    use super::semi_ordered_permutation;

    #[test]
    fn example_one() {
        assert_eq!(semi_ordered_permutation(vec![2, 1, 4, 3]), 2);
    }

    #[test]
    fn example_two() {
        assert_eq!(semi_ordered_permutation(vec![2, 4, 1, 3]), 3);
    }

    #[test]
    fn example_three() {
        assert_eq!(semi_ordered_permutation(vec![1, 3, 4, 2, 5]), 0);
    }
}
