/// LeetCode #1720 - Decode XORed Array
fn decode(encoded: Vec<i32>, first: i32) -> Vec<i32> {
    let mut arr = vec![first];
    for &x in &encoded {
        arr.push(arr.last().unwrap() ^ x);
    }
    arr
}
fn main() { println!("{:?}", decode(vec![1, 2, 3], 1)); }
#[cfg(test)]
mod tests {
    use super::decode;
    #[test]
    fn example_one() {
        assert_eq!(decode(vec![1, 2, 3], 1), vec![1, 0, 2, 1]);
    }
    #[test]
    fn example_two() {
        assert_eq!(decode(vec![6, 2, 7, 3], 4), vec![4, 2, 0, 7, 4]);
    }
}
