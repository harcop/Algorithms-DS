/// LeetCode #3536 - Maximum Product of Two Digits
fn max_product(mut n: i32) -> i32 {
    let mut a = 0;
    let mut b = 0;
    while n > 0 {
        let x = n % 10;
        n /= 10;
        if a < x {
            b = a;
            a = x;
        } else if b < x {
            b = x;
        }
    }
    a * b
}

fn main() {
    println!("{}", max_product(31));
}

#[cfg(test)]
mod tests {
    use super::max_product;

    #[test]
    fn example1() {
        assert_eq!(max_product(31), 3);
    }

    #[test]
    fn example2() {
        assert_eq!(max_product(22), 4);
    }

    #[test]
    fn example3() {
        assert_eq!(max_product(124), 8);
    }
}
