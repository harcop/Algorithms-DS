/// LeetCode #2810 - Faulty Keyboard
fn final_string(s: String) -> String {
    let mut t = Vec::new();
    for c in s.chars() {
        if c == 'i' {
            t.reverse();
        } else {
            t.push(c);
        }
    }
    t.into_iter().collect()
}

fn main() {
    println!("{}", final_string("string".into()));
}

#[cfg(test)]
mod tests {
    use super::final_string;

    #[test]
    fn example_one() {
        assert_eq!(final_string("string".into()), "rtsng");
    }

    #[test]
    fn example_two() {
        assert_eq!(final_string("poiinter".into()), "ponter");
    }
}
