/// LeetCode #343 - Integer Break
fn integer_break(n: i32) -> i32 {
    if n <= 3 {
        return n - 1;
    }
    let mut n = n;
    let mut prod = 1;
    while n > 4 {
        prod *= 3;
        n -= 3;
    }
    prod * n
}

fn main() {
    println!("{}", integer_break(10));
}

#[cfg(test)]
mod tests {
    use super::integer_break;

    #[test]
    fn example_one() {
        assert_eq!(integer_break(10), 36);
    }
}
