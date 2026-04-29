/// LeetCode #80 - Remove Duplicates from Sorted Array II
fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
    if nums.len() <= 2 {
        return nums.len() as i32;
    }
    let mut write = 2usize;
    for read in 2..nums.len() {
        if nums[read] != nums[write - 2] {
            nums[write] = nums[read];
            write += 1;
        }
    }
    write as i32
}

fn main() {
    let mut nums = vec![1, 1, 1, 2, 2, 3];
    let k = remove_duplicates(&mut nums);
    println!("{k}, {:?}", &nums[..k as usize]);
}

#[cfg(test)]
mod tests {
    use super::remove_duplicates;

    #[test]
    fn example_one() {
        let mut nums = vec![1, 1, 1, 2, 2, 3];
        let k = remove_duplicates(&mut nums);
        assert_eq!(k, 5);
        assert_eq!(&nums[..k as usize], [1, 1, 2, 2, 3]);
    }

    #[test]
    fn example_two() {
        let mut nums = vec![0, 0, 1, 1, 1, 1, 2, 3, 3];
        let k = remove_duplicates(&mut nums);
        assert_eq!(k, 7);
        assert_eq!(&nums[..k as usize], [0, 0, 1, 1, 2, 3, 3]);
    }
}
