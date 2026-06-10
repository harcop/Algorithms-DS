/// LeetCode #1812 - Determine Color of a Chessboard Square
fn square_is_white(coordinates: String) -> bool {
    let b = coordinates.as_bytes();
    (b[0] + b[1]) % 2 == 1
}

fn main() {
    println!("{}", square_is_white("a1".into()));
}

#[cfg(test)]
mod tests {
    use super::square_is_white;

    #[test]
    fn example_one() {
        assert!(!square_is_white("a1".into()));
    }

    #[test]
    fn example_two() {
        assert!(square_is_white("h3".into()));
    }
}
