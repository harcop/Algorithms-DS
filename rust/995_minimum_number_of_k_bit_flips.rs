/// LeetCode #995 - Minimum Number of K Bit Flips
fn min_k_bit_flips(nums: Vec<i32>, k: i32) -> i32 {
    let k = k as usize;
    let n = nums.len();
    let mut diff = vec![0i32; n + 1];
    let mut cur = 0i32;
    let mut flips = 0i32;
    for i in 0..n {
        cur += diff[i];
        let bit = (nums[i] + cur) % 2;
        if bit == 0 {
            if i + k > n { return -1; }
            flips += 1;
            cur += 1;
            diff[i + k] -= 1;
        }
    }
    flips
}

fn main() {
    println!("{}", min_k_bit_flips(vec![0, 1, 0], 1));
}

#[cfg(test)]
mod tests {
    use super::min_k_bit_flips;

    #[test]
    fn example_one() {
        assert_eq!(min_k_bit_flips(vec![0, 1, 0], 1), 2);
    }

    #[test]
    fn example_two() {
        assert_eq!(min_k_bit_flips(vec![1, 1, 2], 2), -1);
    }
}
