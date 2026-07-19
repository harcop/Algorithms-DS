/// LeetCode #2507 - Smallest Value After Replacing With Sum of Prime Factors
fn smallest_value(mut n: i32) -> i32 {
    loop {
        let t = n;
        let mut s = 0;
        let mut i = 2;
        while i <= n / i {
            while n % i == 0 {
                s += i;
                n /= i;
            }
            i += 1;
        }
        if n > 1 {
            s += n;
        }
        if s == t {
            return s;
        }
        n = s;
    }
}

fn main() {
    println!("{}", smallest_value(15));
}

#[cfg(test)]
mod tests {
    use super::smallest_value;

    #[test]
    fn example_one() {
        assert_eq!(smallest_value(15), 5);
    }

    #[test]
    fn example_two() {
        assert_eq!(smallest_value(3), 3);
    }
}
