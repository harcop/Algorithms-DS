/// LeetCode #1295 - Find Numbers with Even Number of Digits
fn find_numbers(nums: Vec<i32>) -> i32 {
    nums.iter()
        .filter(|&&x| {
            let mut n = x.abs();
            let mut d = 0;
            if n == 0 {
                return true;
            }
            while n > 0 {
                d += 1;
                n /= 10;
            }
            d % 2 == 0
        })
        .count() as i32
}

fn main() {
    println!("{}", find_numbers(vec![12, 345, 2, 6, 7896]));
}

#[cfg(test)]
mod tests {
    use super::find_numbers;

    #[test]
    fn example_one() {
        assert_eq!(find_numbers(vec![12, 345, 2, 6, 7896]), 2);
    }

    #[test]
    fn example_two() {
        assert_eq!(find_numbers(vec![555, 901, 482, 1771]), 1);
    }
}
