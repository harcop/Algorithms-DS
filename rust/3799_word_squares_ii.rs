/// LeetCode #3799 - Word Squares II
fn word_squares(mut words: Vec<String>) -> Vec<Vec<String>> {
    words.sort();
    let n = words.len();
    let mut ans = Vec::new();
    for i in 0..n {
        for j in 0..n {
            if j == i {
                continue;
            }
            for k in 0..n {
                if k == i || k == j {
                    continue;
                }
                for h in 0..n {
                    if h == i || h == j || h == k {
                        continue;
                    }
                    let top = words[i].as_bytes();
                    let left = words[j].as_bytes();
                    let right = words[k].as_bytes();
                    let bottom = words[h].as_bytes();
                    if top[0] == left[0]
                        && top[3] == right[0]
                        && bottom[0] == left[3]
                        && bottom[3] == right[3]
                    {
                        ans.push(vec![
                            words[i].clone(),
                            words[j].clone(),
                            words[k].clone(),
                            words[h].clone(),
                        ]);
                    }
                }
            }
        }
    }
    ans
}

fn main() {
    println!(
        "{:?}",
        word_squares(vec![
            "able".into(),
            "area".into(),
            "echo".into(),
            "also".into()
        ])
    );
}

#[cfg(test)]
mod tests {
    use super::word_squares;

    #[test]
    fn example1() {
        assert_eq!(
            word_squares(vec![
                "able".into(),
                "area".into(),
                "echo".into(),
                "also".into()
            ]),
            vec![
                vec![
                    "able".to_string(),
                    "area".to_string(),
                    "echo".to_string(),
                    "also".to_string()
                ],
                vec![
                    "area".to_string(),
                    "able".to_string(),
                    "also".to_string(),
                    "echo".to_string()
                ]
            ]
        );
    }

    #[test]
    fn example2() {
        assert_eq!(
            word_squares(vec![
                "code".into(),
                "cafe".into(),
                "eden".into(),
                "edge".into()
            ]),
            Vec::<Vec<String>>::new()
        );
    }
}
