/// LeetCode #3103 - Find Trending Hashtags II (SQL/Pandas; Rust analogue)
use std::collections::HashMap;

fn find_trending_hashtags(tweets: Vec<(i32, i32, String, String)>) -> Vec<(String, i32)> {
    // (user_id, tweet_id, tweet, tweet_date as YYYY-MM-DD)
    let mut counts: HashMap<String, i32> = HashMap::new();

    for (_, _, tweet, date) in tweets {
        if !date.starts_with("2024-02") {
            continue;
        }
        let bytes = tweet.as_bytes();
        let mut i = 0;
        while i < bytes.len() {
            if bytes[i] == b'#' {
                let start = i;
                i += 1;
                while i < bytes.len() && (bytes[i].is_ascii_alphanumeric() || bytes[i] == b'_') {
                    i += 1;
                }
                if i > start + 1 {
                    let tag = String::from_utf8_lossy(&bytes[start..i]).into_owned();
                    *counts.entry(tag).or_default() += 1;
                }
            } else {
                i += 1;
            }
        }
    }

    let mut items: Vec<(String, i32)> = counts.into_iter().collect();
    items.sort_by(|a, b| b.1.cmp(&a.1).then_with(|| b.0.cmp(&a.0)));
    items.into_iter().take(3).collect()
}

fn main() {
    let tweets = vec![
        (
            135,
            13,
            "Enjoying a great start to the day. #HappyDay #MorningVibes".into(),
            "2024-02-01".into(),
        ),
        (
            136,
            14,
            "Another #HappyDay with good vibes! #FeelGood".into(),
            "2024-02-03".into(),
        ),
    ];
    println!("{:?}", find_trending_hashtags(tweets));
}

#[cfg(test)]
mod tests {
    use super::find_trending_hashtags;

    #[test]
    fn example() {
        let tweets = vec![
            (
                135,
                13,
                "Enjoying a great start to the day. #HappyDay #MorningVibes".into(),
                "2024-02-01".into(),
            ),
            (
                136,
                14,
                "Another #HappyDay with good vibes! #FeelGood".into(),
                "2024-02-03".into(),
            ),
            (
                137,
                15,
                "Productivity peaks! #WorkLife #ProductiveDay".into(),
                "2024-02-04".into(),
            ),
            (
                138,
                16,
                "Exploring new tech frontiers. #TechLife #Innovation".into(),
                "2024-02-04".into(),
            ),
            (
                139,
                17,
                "Gratitude for today's moments. #HappyDay #Thankful".into(),
                "2024-02-05".into(),
            ),
            (
                140,
                18,
                "Innovation drives us. #TechLife #FutureTech".into(),
                "2024-02-07".into(),
            ),
            (
                141,
                19,
                "Connecting with nature's serenity. #Nature #Peaceful".into(),
                "2024-02-09".into(),
            ),
        ];
        assert_eq!(
            find_trending_hashtags(tweets),
            vec![
                ("#HappyDay".into(), 3),
                ("#TechLife".into(), 2),
                ("#WorkLife".into(), 1),
            ]
        );
    }
}
