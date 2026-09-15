/// LeetCode #3746 - Minimum String Length After Balanced Removals
fn min_length_after_removals(s: String) -> i32 {
    let a = s.bytes().filter(|&c| c == b'a').count() as i32;
    let b = s.len() as i32 - a;
    (a - b).abs()
}

fn main() {
    println!("{}", min_length_after_removals("aabbab".into()));
}

#[cfg(test)]
mod tests {
    use super::min_length_after_removals;

    #[test]
    fn example1() {
        assert_eq!(min_length_after_removals("aabbab".into()), 0);
    }

    #[test]
    fn example2() {
        assert_eq!(min_length_after_removals("aaaa".into()), 4);
    }

    #[test]
    fn example3() {
        assert_eq!(min_length_after_removals("aaabb".into()), 1);
    }
}
