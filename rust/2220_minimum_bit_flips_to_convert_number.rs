/// LeetCode #2220 - Minimum Bit Flips to Convert Number
fn min_bit_flips(start: i32, goal: i32) -> i32 {
    (start ^ goal).count_ones() as i32
}

fn main() {
    println!("{}", min_bit_flips(10, 7));
}

#[cfg(test)]
mod tests {
    use super::min_bit_flips;

    #[test]
    fn example_one() {
        assert_eq!(min_bit_flips(10, 7), 3);
    }

    #[test]
    fn example_two() {
        assert_eq!(min_bit_flips(3, 4), 3);
    }
}
