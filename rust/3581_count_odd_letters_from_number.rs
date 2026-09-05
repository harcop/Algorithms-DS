/// LeetCode #3581 - Count Odd Letters from Number
fn count_odd_letters(mut n: i32) -> i32 {
    const WORDS: [&str; 10] = [
        "zero", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
    ];
    let mut mask: u32 = 0;
    while n > 0 {
        let x = (n % 10) as usize;
        n /= 10;
        for c in WORDS[x].bytes() {
            mask ^= 1 << (c - b'a');
        }
    }
    mask.count_ones() as i32
}

fn main() {
    println!("{}", count_odd_letters(41));
}

#[cfg(test)]
mod tests {
    use super::count_odd_letters;

    #[test]
    fn example1() {
        assert_eq!(count_odd_letters(41), 5);
    }

    #[test]
    fn example2() {
        assert_eq!(count_odd_letters(20), 5);
    }
}
