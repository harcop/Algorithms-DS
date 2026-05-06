/// LeetCode #371 - Sum of Two Integers
fn get_sum(mut a: i32, mut b: i32) -> i32 {
    while b != 0 {
        let carry = (a & b) << 1;
        a ^= b;
        b = carry;
    }
    a
}

fn main() {
    println!("{}", get_sum(1, 2));
}

#[cfg(test)]
mod tests {
    use super::get_sum;

    #[test]
    fn example_one() {
        assert_eq!(get_sum(1, 2), 3);
    }

    #[test]
    fn example_two() {
        assert_eq!(get_sum(-2, 3), 1);
    }
}
