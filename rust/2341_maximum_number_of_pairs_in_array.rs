/// LeetCode #2341 - Maximum Number of Pairs in Array
fn number_of_pairs(nums: Vec<i32>) -> Vec<i32> {
    let n = nums.len();
    let mut count = [0usize; 101];
    for &v in &nums {
        count[v as usize] += 1;
    }
    let mut sum = 0usize;
    for &v in &count {
        sum += v >> 1;
    }
    vec![sum as i32, (n - sum * 2) as i32]
}

fn main() {
    println!("{:?}", number_of_pairs(vec![1, 3, 2, 1, 3, 2, 2]));
}

#[cfg(test)]
mod tests {
    use super::number_of_pairs;

    #[test]
    fn example_one() {
        assert_eq!(
            number_of_pairs(vec![1, 3, 2, 1, 3, 2, 2]),
            vec![3, 1]
        );
    }

    #[test]
    fn example_two() {
        assert_eq!(number_of_pairs(vec![1, 1]), vec![1, 0]);
    }

    #[test]
    fn example_three() {
        assert_eq!(number_of_pairs(vec![0, 0, 0]), vec![1, 1]);
    }
}
