/// LeetCode #476 - Number Complement
fn find_complement(num: i32) -> i32 {
    if num == 0 {
        return 1;
    }
    let mut mask: i32 = 0;
    let mut x = num;
    while x > 0 {
        mask = (mask << 1) | 1;
        x >>= 1;
    }
    (!num) & mask
}

fn main() {
    println!("{}", find_complement(5));
}

#[cfg(test)]
mod tests {
    use super::find_complement;

    #[test]
    fn example_one() {
        assert_eq!(find_complement(5), 2);
    }

    #[test]
    fn example_two() {
        assert_eq!(find_complement(1), 0);
    }
}
