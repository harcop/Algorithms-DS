/// LeetCode #3870 - Count Commas in Range
fn count_commas(n: i32) -> i32 {
    0.max(n - 999)
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
