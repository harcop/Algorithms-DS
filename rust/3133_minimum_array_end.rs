/// LeetCode #3133 - Minimum Array End
fn min_end(mut n: i32, x: i32) -> i64 {
    n -= 1;
    let mut ans = x as i64;
    let mut n = n as i64;
    for i in 0..31 {
        if ((x >> i) & 1) == 0 {
            ans |= (n & 1) << i;
            n >>= 1;
        }
    }
    ans |= n << 31;
    ans
}

fn main() {
    println!("{}", min_end(3, 4));
}

#[cfg(test)]
mod tests {
    use super::min_end;

    #[test]
    fn example1() {
        assert_eq!(min_end(3, 4), 6);
    }

    #[test]
    fn example2() {
        assert_eq!(min_end(2, 7), 15);
    }
}
