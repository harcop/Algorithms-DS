/// LeetCode #4000 - Largest Integer With Given Digit Sum
fn largest_integer(n: i32, mut s: i32) -> i32 {
    if n * 9 < s {
        return -1;
    }
    let mut ans = 0i32;
    for _ in 0..n {
        let x = s.min(9);
        ans = ans * 10 + x;
        s -= x;
    }
    ans
}

fn main() {
    println!("{}", largest_integer(2, 9));
}

#[cfg(test)]
mod tests {
    use super::largest_integer;

    #[test]
    fn example1() {
        assert_eq!(largest_integer(2, 9), 90);
    }

    #[test]
    fn example2() {
        assert_eq!(largest_integer(2, 19), -1);
    }

    #[test]
    fn example3() {
        assert_eq!(largest_integer(5, 0), 0);
    }
}
