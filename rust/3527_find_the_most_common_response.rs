/// LeetCode #3527 - Find the Most Common Response
use std::collections::{HashMap, HashSet};

fn find_common_response(responses: Vec<Vec<String>>) -> String {
    let mut cnt: HashMap<String, i32> = HashMap::new();
    for ws in &responses {
        let uniq: HashSet<&String> = ws.iter().collect();
        for w in uniq {
            *cnt.entry(w.clone()).or_insert(0) += 1;
        }
    }
    let mut ans = responses[0][0].clone();
    let mut best = cnt.get(&ans).copied().unwrap_or(0);
    for (w, x) in cnt {
        if x > best || (x == best && w < ans) {
            best = x;
            ans = w;
        }
    }
    ans
}

fn main() {
    println!(
        "{}",
        find_common_response(vec![
            vec!["good".into(), "ok".into(), "good".into(), "ok".into()],
            vec!["ok".into(), "bad".into(), "good".into(), "ok".into(), "ok".into()],
            vec!["good".into()],
            vec!["bad".into()],
        ])
    );
}

#[cfg(test)]
mod tests {
    use super::find_common_response;

    #[test]
    fn example1() {
        assert_eq!(
            find_common_response(vec![
                vec!["good".into(), "ok".into(), "good".into(), "ok".into()],
                vec!["ok".into(), "bad".into(), "good".into(), "ok".into(), "ok".into()],
                vec!["good".into()],
                vec!["bad".into()],
            ]),
            "good"
        );
    }

    #[test]
    fn example2() {
        assert_eq!(
            find_common_response(vec![
                vec!["good".into(), "ok".into(), "good".into()],
                vec!["ok".into(), "bad".into()],
                vec!["bad".into(), "notsure".into()],
                vec!["great".into(), "good".into()],
            ]),
            "bad"
        );
    }
}
