/// LeetCode #1567 - Maximum Length Of Subarray With Positive Product
fn get_max_len(nums: Vec<i32>) -> i32 {
    let n = nums.len();
    let mut ans = 0usize;
    let mut i = 0usize;
    while i < n {
        if nums[i] == 0 { i += 1; continue; }
        let start = i;
        let mut negs = vec![];
        while i < n && nums[i] != 0 {
            if nums[i] < 0 { negs.push(i); }
            i += 1;
        }
        let len = i - start;
        if negs.len() % 2 == 0 {
            ans = ans.max(len);
        } else {
            ans = ans.max(len - (negs[0] - start));
            ans = ans.max((i - 1) - negs[negs.len() - 1]);
        }
    }
    ans as i32
}
fn main() { println!("{}", get_max_len(vec![1, -2, -3, 4])); }
#[cfg(test)]
mod tests {
    use super::get_max_len;
    #[test]
    fn example_one() { assert_eq!(get_max_len(vec![1, -2, -3, 4]), 4); }
    #[test]
    fn example_two() { assert_eq!(get_max_len(vec![0, 1, -2, -3, -4]), 3); }
}