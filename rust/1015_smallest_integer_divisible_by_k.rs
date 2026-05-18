/// LeetCode #1015 - Smallest Integer Divisible by K
fn smallest_repunit_div_by_k(k: i32) -> i32 {
    if k % 2 == 0 || k % 5 == 0 {
        return -1;
    }
    let mut rem = 0i32;
    for len in 1..=k {
        rem = (rem * 10 + 1) % k;
        if rem == 0 {
            return len;
        }
    }
    -1
}

fn main() {
    println!("{}", smallest_repunit_div_by_k(3));
}

#[cfg(test)]
mod tests {
    use super::smallest_repunit_div_by_k;

    #[test]
    fn example_one() {
        assert_eq!(smallest_repunit_div_by_k(1), 1);
    }

    #[test]
    fn example_two() {
        assert_eq!(smallest_repunit_div_by_k(2), -1);
    }

    #[test]
    fn example_three() {
        assert_eq!(smallest_repunit_div_by_k(3), 3);
    }
}
