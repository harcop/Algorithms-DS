/// LeetCode #2549 - Count Distinct Numbers on Board
fn distinct_integers(n: i32) -> i32 {
    (n - 1).max(1)
}

fn main() {
    println!("{}", distinct_integers(5));
}

#[cfg(test)]
mod tests {
    use super::distinct_integers;

    #[test]
    fn example_one() {
        assert_eq!(distinct_integers(5), 4);
    }

    #[test]
    fn example_two() {
        assert_eq!(distinct_integers(3), 2);
    }
}
