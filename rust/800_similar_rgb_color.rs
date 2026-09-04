/// LeetCode #800 - Similar RGB Color
fn similar_rgb(color: String) -> String {
    let bytes = color.as_bytes();
    let r = hex_byte(&bytes[1..3]);
    let g = hex_byte(&bytes[3..5]);
    let b = hex_byte(&bytes[5..7]);
    format!("#{:02x}{:02x}{:02x}", nearest(r), nearest(g), nearest(b))
}

fn hex_byte(s: &[u8]) -> i32 {
    let hi = hex_val(s[0]);
    let lo = hex_val(s[1]);
    hi * 16 + lo
}

fn hex_val(c: u8) -> i32 {
    if c.is_ascii_digit() {
        (c - b'0') as i32
    } else {
        (c.to_ascii_lowercase() - b'a' + 10) as i32
    }
}

fn nearest(x: i32) -> i32 {
    let q = ((x as f64) / 17.0).round() as i32;
    (q.clamp(0, 15)) * 17
}

fn main() {
    println!("{}", similar_rgb("#09f166".into()));
}

#[cfg(test)]
mod tests {
    use super::similar_rgb;

    #[test]
    fn example_one() {
        assert_eq!(similar_rgb("#09f166".into()), "#11ee66");
    }

    #[test]
    fn example_two() {
        assert_eq!(similar_rgb("#4e3fe1".into()), "#5544dd");
    }
}
