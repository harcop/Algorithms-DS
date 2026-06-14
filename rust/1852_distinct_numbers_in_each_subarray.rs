/// LeetCode #1852 - Distinct Numbers in Each Subarray
use std::collections::HashMap;

fn distinct_numbers(nums: Vec<i32>, k: i32) -> Vec<i32> {
    let k = k as usize;
    let mut cnt: HashMap<i32, i32> = HashMap::new();
    for &x in &nums[..k] {
        *cnt.entry(x).or_insert(0) += 1;
    }
    let mut ans = vec![cnt.len() as i32];
    for i in k..nums.len() {
        *cnt.entry(nums[i]).or_insert(0) += 1;
        let out = nums[i - k];
        let e = cnt.get_mut(&out).unwrap();
        *e -= 1;
        if *e == 0 {
            cnt.remove(&out);
        }
        ans.push(cnt.len() as i32);
    }
    ans
}

fn main() {
    println!("{:?}", distinct_numbers(vec![1, 2, 3, 2, 2, 1, 3], 3));
}

#[cfg(test)]
mod tests {
    use super::distinct_numbers;

    #[test]
    fn example_one() {
        assert_eq!(
            distinct_numbers(vec![1, 2, 3, 2, 2, 1, 3], 3),
            vec![3, 2, 2, 2, 3]
        );
    }
}
