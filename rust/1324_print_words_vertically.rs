/// LeetCode #1324 - Print Words Vertically
fn print_vertically(s: String) -> Vec<String> {
    let words: Vec<&str> = s.split_whitespace().collect();
    let max_len = words.iter().map(|w| w.len()).max().unwrap_or(0);
    let mut ans = vec![];
    for i in 0..max_len {
        let mut col = String::new();
        for w in &words {
            if i < w.len() {
                col.push(w.as_bytes()[i] as char);
            } else {
                col.push(' ');
            }
        }
        while col.ends_with(' ') {
            col.pop();
        }
        ans.push(col);
    }
    ans
}

fn main() {
    println!("{:?}", print_vertically("TO BE OR NOT TO BE".to_string()));
}

#[cfg(test)]
mod tests {
    use super::print_vertically;

    #[test]
    fn example_one() {
        assert_eq!(
            print_vertically("TO BE OR NOT TO BE".to_string()),
            vec!["TBONTB".to_string(), "OEROOE".to_string(), "   T".to_string()]
        );
    }

    #[test]
    fn example_two() {
        assert_eq!(print_vertically("AB".to_string()), vec!["A".to_string(), "B".to_string()]);
    }
}
