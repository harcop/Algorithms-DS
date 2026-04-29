/// LeetCode #68 - Text Justification
fn full_justify(words: Vec<String>, max_width: i32) -> Vec<String> {
    let max_width = max_width as usize;
    let mut result = Vec::new();
    let mut line: Vec<String> = Vec::new();
    let mut line_len = 0usize;

    for w in words {
        let extra = if line.is_empty() { 0 } else { 1 };
        if line_len + extra + w.len() > max_width {
            result.push(pack_line(&line, max_width, false));
            line.clear();
            line_len = 0;
        }
        line_len += w.len() + if line.is_empty() { 0 } else { 1 };
        line.push(w);
    }
    if !line.is_empty() {
        result.push(pack_line(&line, max_width, true));
    }
    result
}

fn pack_line(words: &[String], max_width: usize, last_line: bool) -> String {
    if words.len() == 1 || last_line {
        let mut s = words.join(" ");
        while s.len() < max_width {
            s.push(' ');
        }
        return s;
    }
    let total_chars: usize = words.iter().map(|w| w.len()).sum();
    let gaps = words.len() - 1;
    let total_spaces = max_width - total_chars;
    let base = total_spaces / gaps;
    let extra = total_spaces % gaps;
    let mut out = String::new();
    for (i, w) in words.iter().enumerate() {
        out.push_str(w);
        if i < gaps {
            let spaces = base + if i < extra { 1 } else { 0 };
            out.extend(std::iter::repeat(' ').take(spaces));
        }
    }
    out
}

fn main() {
    let words = vec![
        "This".to_string(),
        "is".to_string(),
        "an".to_string(),
        "example".to_string(),
        "of".to_string(),
        "text".to_string(),
        "justification.".to_string(),
    ];
    for line in full_justify(words, 16) {
        println!("{line}");
    }
}

#[cfg(test)]
mod tests {
    use super::full_justify;

    #[test]
    fn example_one() {
        let got = full_justify(
            vec![
                "This".to_string(),
                "is".to_string(),
                "an".to_string(),
                "example".to_string(),
                "of".to_string(),
                "text".to_string(),
                "justification.".to_string(),
            ],
            16,
        );
        assert_eq!(
            got,
            vec![
                "This    is    an",
                "example  of text",
                "justification.  "
            ]
        );
    }
}
