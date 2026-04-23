/// LeetCode #27 - Remove Element
fn remove_element(nums: &mut Vec<i32>, val: i32) -> i32 {
    let mut write = 0usize;
    for read in 0..nums.len() {
        if nums[read] != val {
            nums[write] = nums[read];
            write += 1;
        }
    }
    write as i32
}

fn main() {
    let mut nums = vec![3, 2, 2, 3];
    let k = remove_element(&mut nums, 3);
    println!("{k}, {:?}", &nums[..k as usize]);
}

#[cfg(test)]
mod tests {
    use super::remove_element;

    #[test]
    fn example_one() {
        let mut nums = vec![3, 2, 2, 3];
        let k = remove_element(&mut nums, 3);
        assert_eq!(k, 2);
        assert_eq!(&nums[..k as usize], [2, 2]);
    }

    #[test]
    fn example_two() {
        let mut nums = vec![0, 1, 2, 2, 3, 0, 4, 2];
        let k = remove_element(&mut nums, 2);
        let mut got = nums[..k as usize].to_vec();
        got.sort_unstable();
        assert_eq!(k, 5);
        assert_eq!(got, vec![0, 0, 1, 3, 4]);
    }
}
