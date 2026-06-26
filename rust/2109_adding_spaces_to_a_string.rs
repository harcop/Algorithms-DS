/// LeetCode #2109 - Adding Spaces to a String
fn add_spaces(s: String, spaces: Vec<i32>) -> String {
    let bytes = s.as_bytes();
    let mut ans = String::with_capacity(s.len() + spaces.len());
    let mut j = 0usize;
    for i in 0..bytes.len() {
        if j < spaces.len() && spaces[j] as usize == i {
            ans.push(' ');
            j += 1;
        }
        ans.push(bytes[i] as char);
    }
    ans
}

fn main() {
    println!("{}", add_spaces("LeetcodeHelpsMeLearn".into(), vec![8, 13, 15]));
}

#[cfg(test)]
mod tests {
    use super::add_spaces;

    #[test]
    fn example_one() {
        assert_eq!(
            add_spaces("LeetcodeHelpsMeLearn".into(), vec![8, 13, 15]),
            "Leetcode Helps Me Learn"
        );
    }

    #[test]
    fn example_two() {
        assert_eq!(add_spaces("icodeinpython".into(), vec![1, 5, 7, 9]), "i code in py thon");
    }

    #[test]
    fn example_three() {
        assert_eq!(add_spaces("spacing".into(), vec![0, 1, 2, 3, 4, 5, 6]), " s p a c i n g");
    }
}
