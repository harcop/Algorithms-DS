/// LeetCode #3274 - Check if Two Chessboard Squares Have the Same Color
fn check_two_chessboards(coordinate1: String, coordinate2: String) -> bool {
    let a = coordinate1.as_bytes();
    let b = coordinate2.as_bytes();
    let x = a[0] as i32 - b[0] as i32;
    let y = a[1] as i32 - b[1] as i32;
    (x + y) % 2 == 0
}

fn main() {
    println!(
        "{}",
        check_two_chessboards("a1".into(), "c3".into())
    );
}

#[cfg(test)]
mod tests {
    use super::check_two_chessboards;

    #[test]
    fn example1() {
        assert!(check_two_chessboards("a1".into(), "c3".into()));
    }

    #[test]
    fn example2() {
        assert!(!check_two_chessboards("a1".into(), "h3".into()));
    }
}
