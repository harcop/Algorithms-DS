/// LeetCode #2515 - Shortest Distance to Target String in a Circular Array
fn closest_target(words: Vec<String>, target: String, start_index: i32) -> i32 {
    let n = words.len() as i32;
    let mut ans = n;
    for (i, w) in words.iter().enumerate() {
        if *w == target {
            let t = (i as i32 - start_index).abs();
            ans = ans.min(t.min(n - t));
        }
    }
    if ans == n {
        -1
    } else {
        ans
    }
}

fn main() {
    println!(
        "{}",
        closest_target(
            vec![
                "hello".to_string(),
                "i".to_string(),
                "am".to_string(),
                "leetcode".to_string(),
                "hello".to_string()
            ],
            "hello".to_string(),
            1
        )
    );
}

#[cfg(test)]
mod tests {
    use super::closest_target;

    #[test]
    fn example_one() {
        assert_eq!(
            closest_target(
                vec![
                    "hello".to_string(),
                    "i".to_string(),
                    "am".to_string(),
                    "leetcode".to_string(),
                    "hello".to_string()
                ],
                "hello".to_string(),
                1
            ),
            1
        );
    }

    #[test]
    fn example_two() {
        assert_eq!(
            closest_target(
                vec!["a".to_string(), "b".to_string(), "leetcode".to_string()],
                "leetcode".to_string(),
                0
            ),
            1
        );
    }

    #[test]
    fn example_three() {
        assert_eq!(
            closest_target(
                vec!["i".to_string(), "eat".to_string(), "leetcode".to_string()],
                "ate".to_string(),
                0
            ),
            -1
        );
    }
}
