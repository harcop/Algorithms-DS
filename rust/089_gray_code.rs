/// LeetCode #89 - Gray Code
fn gray_code(n: i32) -> Vec<i32> {
    let len = 1i32 << n;
    (0..len).map(|i| i ^ (i >> 1)).collect()
}

fn main() {
    println!("{:?}", gray_code(2));
}

#[cfg(test)]
mod tests {
    use super::gray_code;

    #[test]
    fn example_one() {
        assert_eq!(gray_code(2), vec![0, 1, 3, 2]);
    }

    #[test]
    fn example_two() {
        assert_eq!(gray_code(1), vec![0, 1]);
    }
}
