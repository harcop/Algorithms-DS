/// LeetCode #3876 - Construct Uniform Parity Array II
fn uniform_array(nums1: Vec<i32>) -> bool {
    match nums1.iter().filter(|&&x| x % 2 == 1).min() {
        None => true,
        Some(&mn) => !nums1.iter().any(|&x| x % 2 == 0 && x < mn),
    }
}

fn main() {
    println!("{}", uniform_array(vec![1, 4, 7]));
}

#[cfg(test)]
mod tests {
    use super::uniform_array;

    #[test]
    fn example1() {
        assert_eq!(uniform_array(vec![1, 4, 7]), true);
    }

    #[test]
    fn example2() {
        assert_eq!(uniform_array(vec![2, 3]), false);
    }

    #[test]
    fn example3() {
        assert_eq!(uniform_array(vec![4, 6]), true);
    }
}
