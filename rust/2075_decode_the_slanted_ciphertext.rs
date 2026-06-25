/// LeetCode #2075 - Decode the Slanted Ciphertext
fn decode_ciphertext(encoded_text: String, rows: i32) -> String {
    let rows = rows as usize;
    let bytes = encoded_text.as_bytes();
    let cols = bytes.len() / rows;
    let mut decoded = String::new();

    for col in 0..cols {
        let mut r = 0usize;
        let mut c = col;
        while r < rows && c < cols {
            decoded.push(bytes[r * cols + c] as char);
            r += 1;
            c += 1;
        }
    }

    decoded.trim_end().to_string()
}

fn main() {
    println!("{}", decode_ciphertext("ch   ie   pr".into(), 3));
}

#[cfg(test)]
mod tests {
    use super::decode_ciphertext;

    #[test]
    fn example_one() {
        assert_eq!(decode_ciphertext("ch   ie   pr".into(), 3), "cipher");
    }

    #[test]
    fn example_two() {
        assert_eq!(decode_ciphertext("iveo    eed   l te   olc".into(), 4), "i love leetcode");
    }

    #[test]
    fn example_three() {
        assert_eq!(decode_ciphertext("coding".into(), 1), "coding");
    }
}
