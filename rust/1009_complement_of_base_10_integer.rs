/// LeetCode #1009 - Complement of Base 10 Integer
fn bitwise_complement(n: i32) -> i32 {
    if n == 0 {
        return 1;
    }
    let mut mask = 1i32;
    while mask < n {
        mask = (mask << 1) | 1;
    }
    mask ^ n
}

fn main() {
    println!("{}", bitwise_complement(5));
}

#[cfg(test)]
mod tests {
    use super::bitwise_complement;

    #[test]
    fn example_one() {
        assert_eq!(bitwise_complement(5), 2);
    }

    #[test]
    fn example_two() {
        assert_eq!(bitwise_complement(7), 0);
    }
}
