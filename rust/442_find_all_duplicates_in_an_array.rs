/// LeetCode #442 - Find All Duplicates in an Array
fn find_duplicates(mut nums: Vec<i32>) -> Vec<i32> {
    let mut out = vec![];
    let n = nums.len();
    for i in 0..n {
        let j = nums[i].abs() as usize - 1;
        if nums[j] < 0 {
            out.push((j + 1) as i32);
        } else {
            nums[j] = -nums[j];
        }
    }
    out.sort_unstable();
    out
}

fn main() {
    println!("{:?}", find_duplicates(vec![4, 3, 2, 7, 8, 2, 3, 1]));
}

#[cfg(test)]
mod tests {
    use super::find_duplicates;

    #[test]
    fn example_one() {
        assert_eq!(
            find_duplicates(vec![4, 3, 2, 7, 8, 2, 3, 1]),
            vec![2, 3]
        );
    }
}
