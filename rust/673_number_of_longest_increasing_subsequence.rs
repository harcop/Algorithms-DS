/// LeetCode #673 - Number of Longest Increasing Subsequence
fn find_number_of_lis(nums: Vec<i32>) -> i32 {
    let n = nums.len();
    if n == 0 {
        return 0;
    }
    let mut lengths = vec![1i32; n];
    let mut counts = vec![1i32; n];
    let mut longest = 1i32;
    let mut total = 0i32;
    for i in 0..n {
        for j in 0..i {
            if nums[j] < nums[i] {
                if lengths[j] + 1 > lengths[i] {
                    lengths[i] = lengths[j] + 1;
                    counts[i] = counts[j];
                } else if lengths[j] + 1 == lengths[i] {
                    counts[i] += counts[j];
                }
            }
        }
        longest = longest.max(lengths[i]);
    }
    for i in 0..n {
        if lengths[i] == longest {
            total += counts[i];
        }
    }
    total
}

fn main() {
    println!("{}", find_number_of_lis(vec![1, 3, 5, 4, 7]));
}

#[cfg(test)]
mod tests {
    use super::find_number_of_lis;

    #[test]
    fn example_one() {
        assert_eq!(find_number_of_lis(vec![1, 3, 5, 4, 7]), 2);
    }

    #[test]
    fn example_two() {
        assert_eq!(find_number_of_lis(vec![2, 2, 2, 2, 2]), 5);
    }
}
