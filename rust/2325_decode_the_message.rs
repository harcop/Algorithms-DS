/// LeetCode #2325 - Decode the Message
use std::collections::HashMap;

fn decode_message(key: String, message: String) -> String {
    let mut d: HashMap<u8, char> = HashMap::new();
    for &c in key.as_bytes() {
        if c == b' ' || d.contains_key(&c) {
            continue;
        }
        d.insert(c, char::from_u32(b'a' as u32 + d.len() as u32).unwrap());
    }
    message
        .as_bytes()
        .iter()
        .map(|c| {
            if *c == b' ' {
                ' '
            } else {
                *d.get(c).unwrap()
            }
        })
        .collect()
}

fn main() {
    println!(
        "{}",
        decode_message(
            "the quick brown fox jumps over the lazy dog".to_string(),
            "vkbs bs t suepuv".to_string()
        )
    );
}

#[cfg(test)]
mod tests {
    use super::decode_message;

    #[test]
    fn example_one() {
        assert_eq!(
            decode_message(
                "the quick brown fox jumps over the lazy dog".to_string(),
                "vkbs bs t suepuv".to_string()
            ),
            "this is a secret"
        );
    }

    #[test]
    fn example_two() {
        assert_eq!(
            decode_message(
                "eljuxhpwnyrdgtqkviszcfmabo".to_string(),
                "zwx hnfx lqantp mnoeius ycgk vcnjrdb".to_string()
            ),
            "the five boxing wizards jump quickly"
        );
    }
}
