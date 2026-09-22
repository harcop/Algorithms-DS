/// LeetCode #3875 - Construct Uniform Parity Array I
fn uniform_array(_nums1: Vec<i32>) -> bool {
    true
}

fn main() {
    println!("{}", uniform_array(vec![2, 3]));
}

#[cfg(test)]
mod tests {
    use super::uniform_array;

    #[test]
    fn example1() {
        assert_eq!(uniform_array(vec![2, 3]), true);
    }

    #[test]
    fn example2() {
        assert_eq!(uniform_array(vec![4, 6]), true);
    }
}
