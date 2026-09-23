/// LeetCode #3895 - Count Digit Appearances

fn count_digit_appearances(nums: Vec<i32>, digit: i32) -> i32 {
    let d = digit as u8;
    let mut count = 0;
    for n in nums {
        let mut x = n.abs();
        if x == 0 {
            if d == 0 {
                count += 1;
            }
            continue;
        }
        while x > 0 {
            if (x % 10) as u8 == d {
                count += 1;
            }
            x /= 10;
        }
    }
    count
}

fn main() {
    println!(
        "{}",
        count_digit_appearances(vec![12, 54, 32, 22], 2)
    );
}

#[cfg(test)]
mod tests {
    use super::count_digit_appearances;

    #[test]
    fn example1() {
        assert_eq!(count_digit_appearances(vec![12, 54, 32, 22], 2), 4);
    }

    #[test]
    fn example2() {
        assert_eq!(count_digit_appearances(vec![1, 34, 7], 9), 0);
    }
}
