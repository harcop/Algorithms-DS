/// LeetCode #1683 - Invalid Tweets (SQL; Rust analogue)
fn invalid_tweets(tweets: Vec<(i32, String)>) -> Vec<i32> {
    tweets
        .into_iter()
        .filter(|(_, content)| content.len() > 15)
        .map(|(id, _)| id)
        .collect()
}

fn main() {
    println!("{:?}", invalid_tweets(vec![]));
}

#[cfg(test)]
mod tests {
    use super::invalid_tweets;

    #[test]
    fn example() {
        let tweets = vec![
            (1, "Let us Code".into()),
            (2, "More than fifteen chars are here!".into()),
        ];
        assert_eq!(invalid_tweets(tweets), vec![2]);
    }
}
