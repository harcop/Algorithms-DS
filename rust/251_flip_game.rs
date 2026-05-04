/// LeetCode #251 - Flip Game
fn generate_possible_next_moves(current: String) -> Vec<String> {
    let b = current.as_bytes();
    let mut out = vec![];
    for i in 0..b.len().saturating_sub(1) {
        if b[i] == b'+' && b[i + 1] == b'+' {
            let mut ch: Vec<char> = current.chars().collect();
            ch[i] = '-';
            ch[i + 1] = '-';
            out.push(ch.into_iter().collect());
        }
    }
    out
}

fn main() {
    println!("{:?}", generate_possible_next_moves("++++".into()));
}

#[cfg(test)]
mod tests {
    use super::generate_possible_next_moves;

    #[test]
    fn example_one() {
        let mut v = generate_possible_next_moves("++++".into());
        v.sort();
        let mut e: Vec<String> = vec!["--++".into(), "+--+".into(), "++--".into()];
        e.sort();
        assert_eq!(v, e);
    }
}
