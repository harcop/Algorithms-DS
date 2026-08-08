/// LeetCode #3087 - Find Trending Hashtags (SQL; Rust analogue)
use std::collections::HashMap;

fn find_trending_hashtags(tweets: Vec<(i32, i32, String, String)>) -> Vec<(String, i32)> {
    // (user_id, tweet_id, tweet, tweet_date as YYYY-MM-DD)
    let mut counts: HashMap<String, i32> = HashMap::new();

    for (_, _, tweet, date) in tweets {
        if !date.starts_with("2024-02") {
            continue;
        }
        if let Some(start) = tweet.find('#') {
            let rest = &tweet[start..];
            let end = rest
                .find(|c: char| c.is_whitespace())
                .unwrap_or(rest.len());
            let tag = rest[..end].to_string();
            *counts.entry(tag).or_default() += 1;
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
            "Enjoying a great start to the day! #HappyDay".into(),
            "2024-02-01".into(),
        ),
        (
            136,
            14,
            "Another #HappyDay with good vibes!".into(),
            "2024-02-03".into(),
        ),
        (
            137,
            15,
            "Productivity peaks! #WorkLife".into(),
            "2024-02-04".into(),
        ),
        (
            138,
            16,
            "Exploring new tech frontiers. #TechLife".into(),
            "2024-02-04".into(),
        ),
        (
            139,
            17,
            "Gratitude for today's moments. #HappyDay".into(),
            "2024-02-05".into(),
        ),
        (
            140,
            18,
            "Innovation drives us. #TechLife".into(),
            "2024-02-07".into(),
        ),
        (
            141,
            19,
            "Connecting with nature's serenity. #Nature".into(),
            "2024-02-09".into(),
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
                "Enjoying a great start to the day! #HappyDay".into(),
                "2024-02-01".into(),
            ),
            (
                136,
                14,
                "Another #HappyDay with good vibes!".into(),
                "2024-02-03".into(),
            ),
            (
                137,
                15,
                "Productivity peaks! #WorkLife".into(),
                "2024-02-04".into(),
            ),
            (
                138,
                16,
                "Exploring new tech frontiers. #TechLife".into(),
                "2024-02-04".into(),
            ),
            (
                139,
                17,
                "Gratitude for today's moments. #HappyDay".into(),
                "2024-02-05".into(),
            ),
            (
                140,
                18,
                "Innovation drives us. #TechLife".into(),
                "2024-02-07".into(),
            ),
            (
                141,
                19,
                "Connecting with nature's serenity. #Nature".into(),
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
