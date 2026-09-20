/// LeetCode #3827 - Count Monobit Integers
fn count_monobit(n: i32) -> i32 {
    let mut ans = 1;
    let mut x = 1i32;
    let mut i = 1;
    while x <= n {
        ans += 1;
        x += 1 << i;
        i += 1;
    }
    ans
}

fn main() {
    println!("{}", count_monobit(1));
}

#[cfg(test)]
mod tests {
    use super::count_monobit;

    #[test]
    fn example1() {
        assert_eq!(count_monobit(1), 2);
    }

    #[test]
    fn example2() {
        assert_eq!(count_monobit(4), 3);
    }
}
