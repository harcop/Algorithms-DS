/// LeetCode #2457 - Minimum Addition to Make Integer Beautiful
fn make_integer_beautiful(n: i64, target: i32) -> i64 {
    fn digit_sum(mut value: i64) -> i32 {
        let mut sum = 0;
        while value > 0 {
            sum += (value % 10) as i32;
            value /= 10;
        }
        sum
    }

    let original = n;
    let mut value = n;
    let mut place = 1i64;

    while digit_sum(value) > target {
        let digit = value / place % 10;
        if digit != 0 {
            value += (10 - digit) * place;
        }
        place *= 10;
    }

    value - original
}

fn main() {
    println!("{}", make_integer_beautiful(16, 6));
}

#[cfg(test)]
mod tests {
    use super::make_integer_beautiful;

    #[test]
    fn example_one() {
        assert_eq!(make_integer_beautiful(16, 6), 4);
    }

    #[test]
    fn example_two() {
        assert_eq!(make_integer_beautiful(467, 6), 33);
    }

    #[test]
    fn already_beautiful() {
        assert_eq!(make_integer_beautiful(1, 1), 0);
    }
}
