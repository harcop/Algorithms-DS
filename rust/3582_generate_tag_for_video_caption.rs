/// LeetCode #3582 - Generate Tag for Video Caption
fn generate_tag(caption: String) -> String {
    let mut words: Vec<String> = caption
        .split_whitespace()
        .map(|w| {
            let mut s = w.to_lowercase();
            if let Some(first) = s.get_mut(0..1) {
                first.make_ascii_uppercase();
            }
            s
        })
        .collect();
    if let Some(first) = words.first_mut() {
        first.make_ascii_lowercase();
    }
    let mut ans = String::from("#");
    ans.push_str(&words.concat());
    ans.truncate(100);
    ans
}

fn main() {
    println!("{}", generate_tag("Leetcode daily streak achieved".into()));
}

#[cfg(test)]
mod tests {
    use super::generate_tag;

    #[test]
    fn example1() {
        assert_eq!(
            generate_tag("Leetcode daily streak achieved".into()),
            "#leetcodeDailyStreakAchieved"
        );
    }

    #[test]
    fn example2() {
        assert_eq!(generate_tag("can I Go There".into()), "#canIGoThere");
    }

    #[test]
    fn example3() {
        let caption = "h".repeat(101);
        let ans = generate_tag(caption);
        assert_eq!(ans.len(), 100);
        assert!(ans.starts_with('#'));
        assert_eq!(ans.chars().filter(|&c| c == 'h').count(), 99);
    }
}
