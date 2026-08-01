/// LeetCode #2868 - The Wording Game
fn can_alice_win(a: Vec<String>, b: Vec<String>) -> bool {
    fn is_valid_next(word: &str, previous: &str) -> bool {
        let first = word.as_bytes()[0];
        let previous_first = previous.as_bytes()[0];
        (first == previous_first && word > previous) || first == previous_first + 1
    }

    let mut alice_index = 1;
    let mut bob_index = 0;
    let mut word = &a[0];
    let mut bob_turn = true;

    loop {
        if bob_turn {
            if bob_index == b.len() {
                return true;
            }
            if is_valid_next(&b[bob_index], word) {
                word = &b[bob_index];
                bob_turn = false;
            }
            bob_index += 1;
        } else {
            if alice_index == a.len() {
                return false;
            }
            if is_valid_next(&a[alice_index], word) {
                word = &a[alice_index];
                bob_turn = true;
            }
            alice_index += 1;
        }
    }
}

fn main() {
    let alice = vec!["ananas", "atlas", "banana"]
        .into_iter()
        .map(String::from)
        .collect();
    let bob = vec!["albatros", "cikla", "nogomet"]
        .into_iter()
        .map(String::from)
        .collect();
    println!("{}", can_alice_win(alice, bob));
}

#[cfg(test)]
mod tests {
    use super::can_alice_win;

    fn words(values: &[&str]) -> Vec<String> {
        values.iter().map(|word| (*word).to_string()).collect()
    }

    #[test]
    fn example_one() {
        assert!(!can_alice_win(
            words(&["avokado", "dabar"]),
            words(&["brazil"])
        ));
    }

    #[test]
    fn example_two() {
        assert!(can_alice_win(
            words(&["ananas", "atlas", "banana"]),
            words(&["albatros", "cikla", "nogomet"])
        ));
    }

    #[test]
    fn example_three() {
        assert!(can_alice_win(
            words(&["hrvatska", "zastava"]),
            words(&["bijeli", "galeb"])
        ));
    }
}
