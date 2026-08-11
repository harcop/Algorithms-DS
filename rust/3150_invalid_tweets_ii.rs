/// LeetCode #3150 - Invalid Tweets II (SQL/Pandas; Rust analogue)
fn find_invalid_tweets(tweets: Vec<(i32, String)>) -> Vec<i32> {
    let mut ans: Vec<i32> = tweets
        .into_iter()
        .filter(|(_, content)| {
            content.len() > 140
                || content.matches('@').count() > 3
                || content.matches('#').count() > 3
        })
        .map(|(id, _)| id)
        .collect();
    ans.sort_unstable();
    ans
}

fn main() {
    let tweets = vec![
        (
            1,
            "Traveling, exploring, and living my best life @JaneSmith @SaraJohnson @LisaTaylor @MikeBrown #Foodie #Fitness #Learning".into(),
        ),
        (
            2,
            "Just had the best dinner with friends! #Foodie #Friends #Fun".into(),
        ),
        (
            4,
            "Working hard on my new project #Work #Goals #Productivity #Fun".into(),
        ),
    ];
    println!("{:?}", find_invalid_tweets(tweets));
}

#[cfg(test)]
mod tests {
    use super::find_invalid_tweets;

    #[test]
    fn example() {
        let tweets = vec![
            (
                1,
                "Traveling, exploring, and living my best life @JaneSmith @SaraJohnson @LisaTaylor @MikeBrown #Foodie #Fitness #Learning".into(),
            ),
            (
                2,
                "Just had the best dinner with friends! #Foodie #Friends #Fun".into(),
            ),
            (
                4,
                "Working hard on my new project #Work #Goals #Productivity #Fun".into(),
            ),
        ];
        assert_eq!(find_invalid_tweets(tweets), vec![1, 4]);
    }
}
