/// LeetCode #2161 - Partition Array According to Given Pivot
fn pivot_array(nums: Vec<i32>, pivot: i32) -> Vec<i32> {
    let mut ans = Vec::with_capacity(nums.len());
    ans.extend(nums.iter().copied().filter(|&x| x < pivot));
    ans.extend(nums.iter().copied().filter(|&x| x == pivot));
    ans.extend(nums.iter().copied().filter(|&x| x > pivot));
    ans
}

fn main() {
    println!("{:?}", pivot_array(vec![9, 12, 5, 10, 14, 3, 10], 10));
}

#[cfg(test)]
mod tests {
    use super::pivot_array;

    #[test]
    fn example_one() {
        assert_eq!(
            pivot_array(vec![9, 12, 5, 10, 14, 3, 10], 10),
            vec![9, 5, 3, 10, 10, 12, 14]
        );
    }

    #[test]
    fn example_two() {
        assert_eq!(pivot_array(vec![-3, 4, 3, 2], 2), vec![-3, 2, 4, 3]);
    }
}
