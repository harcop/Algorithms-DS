/// LeetCode #3498 - Reverse Degree of a String
fn reverse_degree(s: String) -> i32 {
    s.bytes()
        .enumerate()
        .map(|(i, c)| (i as i32 + 1) * (26 - (c - b'a') as i32))
        .sum()
}

fn main() {
    println!("{}", reverse_degree("abc".into()));
}

#[cfg(test)]
mod tests {
    use super::reverse_degree;

    #[test]
    fn example1() {
        assert_eq!(reverse_degree("abc".into()), 148);
    }

    #[test]
    fn example2() {
        assert_eq!(reverse_degree("zaza".into()), 160);
    }
}
