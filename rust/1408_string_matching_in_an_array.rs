/// LeetCode #1408 - String Matching In An Array
fn string_matching(words: Vec<String>) -> Vec<String> {
    let mut ans = Vec::new();
    for i in 0..words.len() {
        for j in 0..words.len() {
            if i != j && words[j].contains(&words[i]) {
                ans.push(words[i].clone());
                break;
            }
        }
    }
    ans
}

fn main() {
    println!("{:?}", string_matching(vec!["leetcode".into(), "et".into(), "code".into()]));
}

#[cfg(test)]
mod tests {
    use super::string_matching;

    #[test]
    fn example_one() {
        let mut out = string_matching(vec!["leetcode".into(), "et".into(), "code".into()]);
        out.sort();
        assert_eq!(out, vec!["code".to_string(), "et".to_string()]);
    }
}

