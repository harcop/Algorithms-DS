/// LeetCode #26 - Remove Duplicates from Sorted Array
fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
    if nums.is_empty() {
        return 0;
    }

    let mut write = 1usize;
    for read in 1..nums.len() {
        if nums[read] != nums[read - 1] {
            nums[write] = nums[read];
            write += 1;
        }
    }

    write as i32
}

fn main() {
    let mut nums = vec![1, 1, 2];
    let k = remove_duplicates(&mut nums);
    println!("{k}, {:?}", &nums[..k as usize]);
}

#[cfg(test)]
mod tests {
    use super::remove_duplicates;

    #[test]
    fn example_one() {
        let mut nums = vec![1, 1, 2];
        let k = remove_duplicates(&mut nums);
        assert_eq!(k, 2);
        assert_eq!(&nums[..k as usize], [1, 2]);
    }

    #[test]
    fn example_two() {
        let mut nums = vec![0, 0, 1, 1, 1, 2, 2, 3, 3, 4];
        let k = remove_duplicates(&mut nums);
        assert_eq!(k, 5);
        assert_eq!(&nums[..k as usize], [0, 1, 2, 3, 4]);
    }
}
