/// LeetCode #1678 - Goal Parser Interpretation
fn interpret(command: String) -> String {
    command.replace("()", "o").replace("(al)", "al")
}
fn main() { println!("{}", interpret("G()(al)".into())); }
#[cfg(test)]
mod tests {
    use super::interpret;
    #[test]
    fn example_one() { assert_eq!(interpret("G()(al)".into()), "Goal"); }
    #[test]
    fn example_two() { assert_eq!(interpret("G()()()()(al)".into()), "Gooooal"); }
}