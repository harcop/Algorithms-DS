/// LeetCode #3345 - Smallest Divisible Digit Product I
fn smallest_number(n: i32, t: i32) -> i32 {
    let mut i = n;
    loop {
        let mut p = 1;
        let mut x = i;
        while x > 0 {
            p *= x % 10;
            x /= 10;
        }
        if p % t == 0 {
            return i;
        }
        i += 1;
    }
}

fn main() {
    println!("{}", smallest_number(10, 2));
}

#[cfg(test)]
mod tests {
    use super::smallest_number;

    #[test]
    fn example1() {
        assert_eq!(smallest_number(10, 2), 10);
    }

    #[test]
    fn example2() {
        assert_eq!(smallest_number(15, 3), 16);
    }
}
