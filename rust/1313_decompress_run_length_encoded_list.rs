/// LeetCode #1313 - Decompress Run-Length Encoded List
fn decompress_rlenc_list(nums: Vec<i32>) -> Vec<i32> {
    let mut ans = vec![];
    let mut i = 0;
    while i < nums.len() {
        let freq = nums[i] as usize;
        let val = nums[i + 1];
        ans.extend(std::iter::repeat(val).take(freq));
        i += 2;
    }
    ans
}

fn main() {
    println!("{:?}", decompress_rlenc_list(vec![1, 2, 3, 4]));
}

#[cfg(test)]
mod tests {
    use super::decompress_rlenc_list;

    #[test]
    fn example_one() {
        assert_eq!(decompress_rlenc_list(vec![1, 2, 3, 4]), vec![2, 4, 4, 4]);
    }

    #[test]
    fn example_two() {
        assert_eq!(decompress_rlenc_list(vec![1, 1, 6, 6]), vec![1, 6, 6, 6, 6, 6, 6]);
    }
}
