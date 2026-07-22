/// LeetCode #2571 - Minimum Operations to Reduce an Integer to 0
fn min_operations(mut n: i32) -> i32 {
    let mut ans = 0;
    while n > 0 {
        if (n & 3) == 3 {
            n += 1;
            ans += 1;
        } else if n % 2 == 1 {
            n -= 1;
            ans += 1;
        } else {
            n >>= 1;
        }
    }
    ans
}

fn main() {
    println!("{}", min_operations(39));
}

#[cfg(test)]
mod tests {
    use super::min_operations;

    #[test]
    fn example_one() {
        assert_eq!(min_operations(39), 3);
    }

    #[test]
    fn example_two() {
        assert_eq!(min_operations(54), 3);
    }
}
