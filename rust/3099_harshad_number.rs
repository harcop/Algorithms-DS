/// LeetCode #3099 - Harshad Number
fn sum_of_the_digits_of_harshad_number(x: i32) -> i32 {
    let mut s = 0;
    let mut y = x;
    while y > 0 {
        s += y % 10;
        y /= 10;
    }
    if x % s == 0 {
        s
    } else {
        -1
    }
}

fn main() {
    println!("{}", sum_of_the_digits_of_harshad_number(18));
}

#[cfg(test)]
mod tests {
    use super::sum_of_the_digits_of_harshad_number;

    #[test]
    fn example1() {
        assert_eq!(sum_of_the_digits_of_harshad_number(18), 9);
    }

    #[test]
    fn example2() {
        assert_eq!(sum_of_the_digits_of_harshad_number(23), -1);
    }
}
