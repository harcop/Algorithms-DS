/// LeetCode #3513 - Number of Unique XOR Triplets I
fn unique_xor_triplets(nums: Vec<i32>) -> i32 {
    let n = nums.len();
    if n <= 2 {
        n as i32
    } else {
        1 << (32 - (n as u32).leading_zeros())
    }
}

fn main() {
    println!("{}", unique_xor_triplets(vec![1, 2]));
}

#[cfg(test)]
mod tests {
    use super::unique_xor_triplets;

    #[test]
    fn example1() {
        assert_eq!(unique_xor_triplets(vec![1, 2]), 2);
    }

    #[test]
    fn example2() {
        assert_eq!(unique_xor_triplets(vec![3, 1, 2]), 4);
    }
}
