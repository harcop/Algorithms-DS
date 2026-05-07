/// LeetCode #448 - Find All Numbers Disappeared in an Array
fn find_disappeared_numbers(mut nums: Vec<i32>) -> Vec<i32> {
    let n = nums.len();
    for i in 0..n {
        let j = nums[i].abs() as usize - 1;
        if nums[j] > 0 {
            nums[j] = -nums[j];
        }
    }
    let mut out = vec![];
    for i in 0..n {
        if nums[i] > 0 {
            out.push((i + 1) as i32);
        }
    }
    out
}

fn main() {
    println!("{:?}", find_disappeared_numbers(vec![4, 3, 2, 7, 8, 2, 3, 1]));
}

#[cfg(test)]
mod tests {
    use super::find_disappeared_numbers;

    #[test]
    fn example_one() {
        let mut v = find_disappeared_numbers(vec![4, 3, 2, 7, 8, 2, 3, 1]);
        v.sort_unstable();
        assert_eq!(v, vec![5, 6]);
    }
}
