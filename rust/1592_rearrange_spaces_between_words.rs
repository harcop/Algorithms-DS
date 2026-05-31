/// LeetCode #1592 - Rearrange Spaces Between Words
fn reorder_spaces(text: String) -> String {
    let words: Vec<&str> = text.split_whitespace().collect();
    let spaces = text.chars().filter(|&c| c == ' ').count();
    if words.len() <= 1 {
        return format!("{}{}", words.join(""), " ".repeat(spaces));
    }
    let between = spaces / (words.len() - 1);
    let extra = spaces % (words.len() - 1);
    let mut out = String::new();
    for (i, w) in words.iter().enumerate() {
        out.push_str(w);
        if i + 1 < words.len() { out.push_str(&" ".repeat(between)); }
    }
    out.push_str(&" ".repeat(extra));
    out
}
fn main() { println!("{}", reorder_spaces("  this   is  a sentence ".into())); }
#[cfg(test)]
mod tests {
    use super::reorder_spaces;
    #[test]
    fn example_one() { assert_eq!(reorder_spaces("  this   is  a sentence ".into()), "this   is   a   sentence"); }
}