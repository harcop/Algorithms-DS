/// LeetCode #434 - Number of Segments in a String
fn count_segments(s: String) -> i32 {
    s.split_whitespace().count() as i32
}

fn main() {
    println!("{}", count_segments("Hello, my name is John".into()));
}

#[cfg(test)]
mod tests {
    use super::count_segments;

    #[test]
    fn example_one() {
        assert_eq!(count_segments("Hello, my name is John".into()), 5);
    }
}
