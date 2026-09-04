/// LeetCode #555 - Split Concatenated Strings
fn split_looped_string(strs: Vec<String>) -> String {
    let best: Vec<String> = strs
        .iter()
        .map(|s| {
            let rev: String = s.chars().rev().collect();
            if s >= &rev { s.clone() } else { rev }
        })
        .collect();
    let mut ans = String::new();
    for i in 0..strs.len() {
        let mid: String = best[i + 1..]
            .iter()
            .chain(best[..i].iter())
            .cloned()
            .collect();
        for body in [
            strs[i].clone(),
            strs[i].chars().rev().collect::<String>(),
        ] {
            for j in 0..body.len() {
                let cand = format!("{}{}{}", &body[j..], mid, &body[..j]);
                if cand > ans {
                    ans = cand;
                }
            }
        }
    }
    ans
}

fn main() {
    let strs = vec!["abc".into(), "xyz".into()];
    println!("{}", split_looped_string(strs));
}

#[cfg(test)]
mod tests {
    use super::split_looped_string;

    #[test]
    fn example() {
        let strs = vec!["abc".into(), "xyz".into()];
        assert_eq!(split_looped_string(strs), "zyxcba");
    }
}
