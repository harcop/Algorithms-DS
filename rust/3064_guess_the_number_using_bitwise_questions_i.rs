/// LeetCode #3064 - Guess the Number Using Bitwise Questions I (interactive analogue)
fn find_number(common_set_bits: impl Fn(i32) -> i32) -> i32 {
    let mut result = 0i32;
    for i in 0..32 {
        if common_set_bits(1 << i) == 1 {
            result |= 1 << i;
        }
    }
    result
}

fn find_number_from_secret(n: i32) -> i32 {
    find_number(|num| (n & num).count_ones() as i32)
}

fn main() {
    println!("{}", find_number_from_secret(31));
    println!("{}", find_number_from_secret(33));
}

#[cfg(test)]
mod tests {
    use super::find_number_from_secret;

    #[test]
    fn example1() {
        assert_eq!(find_number_from_secret(31), 31);
    }

    #[test]
    fn example2() {
        assert_eq!(find_number_from_secret(33), 33);
    }
}
