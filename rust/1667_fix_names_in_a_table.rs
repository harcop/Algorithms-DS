/// LeetCode #1667 - Fix Names in a Table (SQL; Rust analogue)
fn fix_names(users: Vec<(i32, String)>) -> Vec<(i32, String)> {
    let mut ans: Vec<(i32, String)> = users
        .into_iter()
        .map(|(id, name)| {
            let mut cs: Vec<char> = name.chars().collect();
            for (i, c) in cs.iter_mut().enumerate() {
                if i == 0 {
                    *c = c.to_ascii_uppercase();
                } else {
                    *c = c.to_ascii_lowercase();
                }
            }
            (id, cs.into_iter().collect())
        })
        .collect();
    ans.sort_by_key(|r| r.0);
    ans
}

fn main() {
    println!("{:?}", fix_names(vec![]));
}

#[cfg(test)]
mod tests {
    use super::fix_names;

    #[test]
    fn example() {
        let users = vec![(1, "aLice".into()), (2, "bOB".into())];
        assert_eq!(
            fix_names(users),
            vec![(1, "Alice".into()), (2, "Bob".into())]
        );
    }
}
