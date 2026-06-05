/// LeetCode #1734 - Decode XORed Permutation
fn decode(encoded: Vec<i32>) -> Vec<i32> {
    let n = encoded.len() + 1;
    let mut a = 0i32;
    let mut b = 0i32;
    for i in (0..n - 1).step_by(2) {
        a ^= encoded[i];
    }
    for i in 1..=n as i32 {
        b ^= i;
    }
    let mut perm = vec![0i32; n];
    perm[n - 1] = a ^ b;
    for i in (0..n - 1).rev() {
        perm[i] = encoded[i] ^ perm[i + 1];
    }
    perm
}
fn main() { println!("{:?}", decode(vec![3, 1])); }
#[cfg(test)]
mod tests {
    use super::decode;
    #[test]
    fn example_one() {
        assert_eq!(decode(vec![3, 1]), vec![1, 2, 3]);
    }
    #[test]
    fn example_two() {
        assert_eq!(decode(vec![6, 5, 4, 6]), vec![2, 4, 1, 5, 3]);
    }
}
