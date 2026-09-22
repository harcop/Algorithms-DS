/// LeetCode #3871 - Count Commas in Range II
fn count_commas(n: i64) -> i64 {
    let mut ans = 0i64;
    let mut x = 1000i64;
    while x <= n {
        ans += n - x + 1;
        x *= 1000;
    }
    ans
}

fn main() {
    println!("{}", count_commas(1002));
}

#[cfg(test)]
mod tests {
    use super::count_commas;

    #[test]
    fn example1() {
        assert_eq!(count_commas(1002), 3);
    }

    #[test]
    fn example2() {
        assert_eq!(count_commas(998), 0);
    }
}
