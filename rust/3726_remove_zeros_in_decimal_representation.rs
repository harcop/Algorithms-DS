/// LeetCode #3726 - Remove Zeros in Decimal Representation
fn remove_zeros(mut n: i64) -> i64 {
    let mut k = 1i64;
    let mut ans = 0i64;
    while n > 0 {
        let x = n % 10;
        if x > 0 {
            ans += k * x;
            k *= 10;
        }
        n /= 10;
    }
    ans
}

fn main() {
    println!("{}", remove_zeros(1020030));
}

#[cfg(test)]
mod tests {
    use super::remove_zeros;

    #[test]
    fn example1() {
        assert_eq!(remove_zeros(1020030), 123);
    }

    #[test]
    fn example2() {
        assert_eq!(remove_zeros(1), 1);
    }
}
