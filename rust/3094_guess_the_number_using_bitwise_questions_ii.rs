/// LeetCode #3094 - Guess the Number Using Bitwise Questions II (interactive analogue)
fn find_number(mut common_bits: impl FnMut(i32) -> i32) -> i32 {
    let mut n = 0i32;
    for i in 0..30 {
        let count1 = common_bits(1 << i);
        let count2 = common_bits(1 << i);
        if count1 > count2 {
            n |= 1 << i;
        }
    }
    n
}

fn find_number_from_secret(secret: i32) -> i32 {
    let mut n = secret;
    find_number(|num| {
        let count = (!(n ^ num) & ((1 << 30) - 1)).count_ones() as i32;
        n ^= num;
        count
    })
}

fn main() {
    println!("{}", find_number_from_secret(31));
    println!("{}", find_number_from_secret(33));
}

#[cfg(test)]
mod tests {
    use super::find_number_from_secret;

    #[test]
    fn example_values() {
        assert_eq!(find_number_from_secret(0), 0);
        assert_eq!(find_number_from_secret(31), 31);
        assert_eq!(find_number_from_secret(33), 33);
        assert_eq!(find_number_from_secret((1 << 30) - 1), (1 << 30) - 1);
    }
}
