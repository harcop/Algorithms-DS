/// LeetCode #1618 - Maximum Font To Fit A Sentence In A Screen
fn max_font(sentence: String, fonts: Vec<Vec<i32>>, chars: String, w: i32, h: i32) -> i32 {
    let mut best = -1i32;
    for (idx, font) in fonts.iter().enumerate() {
        let mut width = 0i32;
        let mut height = 0i32;
        let mut ok = true;
        for c in sentence.chars() {
            if c == ' ' {
                width += font[0];
                height = height.max(font[1]);
                continue;
            }
            let pos = match chars.find(c) {
                Some(p) => p,
                None => {
                    ok = false;
                    break;
                }
            };
            width += font[2 + pos * 2];
            height = height.max(font[2 + pos * 2 + 1]);
        }
        if ok && width <= w && height <= h {
            best = best.max(idx as i32);
        }
    }
    best
}

fn main() {
    println!(
        "{}",
        max_font("dog".into(), vec![vec![1, 1, 1, 1, 1, 1, 1, 1]], "god".into(), 10, 5)
    );
}

#[cfg(test)]
mod tests {
    use super::max_font;

    #[test]
    fn example_one() {
        assert_eq!(
            max_font("dog".into(), vec![vec![1, 1, 1, 1, 1, 1, 1, 1]], "god".into(), 10, 5),
            0
        );
    }
}
