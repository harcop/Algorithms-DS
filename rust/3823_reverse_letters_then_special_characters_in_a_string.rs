/// LeetCode #3823 - Reverse Letters Then Special Characters in a String
fn reverse_by_type(s: String) -> String {
    let mut t: Vec<u8> = s.into_bytes();
    let mut letters: Vec<u8> = t.iter().copied().filter(|c| c.is_ascii_lowercase()).collect();
    let mut specials: Vec<u8> = t.iter().copied().filter(|c| !c.is_ascii_lowercase()).collect();
    for c in t.iter_mut() {
        if c.is_ascii_lowercase() {
            *c = letters.pop().unwrap();
        } else {
            *c = specials.pop().unwrap();
        }
    }
    String::from_utf8(t).unwrap()
}

fn main() {
    println!("{}", reverse_by_type(")ebc#da@f(".into()));
}

#[cfg(test)]
mod tests {
    use super::reverse_by_type;

    #[test]
    fn example1() {
        assert_eq!(
            reverse_by_type(")ebc#da@f(".into()),
            "(fad@cb#e)"
        );
    }

    #[test]
    fn example2() {
        assert_eq!(reverse_by_type("z".into()), "z");
    }

    #[test]
    fn example3() {
        assert_eq!(
            reverse_by_type("!@#$%^&*()".into()),
            ")(*&^%$#@!"
        );
    }
}
