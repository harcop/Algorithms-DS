/// LeetCode #1348 - Tweet Counts Per Frequency

use std::collections::HashMap;

struct TweetCounts {
    data: HashMap<String, Vec<i32>>,
}

impl TweetCounts {
    fn new() -> Self {
        Self { data: HashMap::new() }
    }

    fn record_tweet(&mut self, tweet_name: String, time: i32) {
        self.data.entry(tweet_name).or_default().push(time);
    }

    fn get_tweet_counts_per_frequency(&self, tweet_name: String, freq: String) -> Vec<i32> {
        let delta = match freq.as_str() {
            "minute" => 60,
            "hour" => 3600,
            _ => 86400,
        };
        let times = self.data.get(&tweet_name).cloned().unwrap_or_default();
        if times.is_empty() {
            return vec![];
        }
        let min_t = *times.iter().min().unwrap();
        let max_t = *times.iter().max().unwrap();
        let buckets = (max_t - min_t) / delta + 1;
        let mut counts = vec![0i32; buckets as usize];
        for t in times {
            let idx = ((t - min_t) / delta) as usize;
            counts[idx] += 1;
        }
        counts
    }
}

fn main() {
    let mut tc = TweetCounts::new();
    tc.record_tweet("tweet3".into(), 0);
    tc.record_tweet("tweet3".into(), 60);
    tc.record_tweet("tweet3".into(), 10);
    println!("{:?}", tc.get_tweet_counts_per_frequency("tweet3".into(), "minute".into()));
}

#[cfg(test)]
mod tests {
    use super::TweetCounts;

    #[test]
    fn example_one() {
        let mut tc = TweetCounts::new();
        tc.record_tweet("tweet3".into(), 0);
        tc.record_tweet("tweet3".into(), 60);
        tc.record_tweet("tweet3".into(), 10);
        assert_eq!(tc.get_tweet_counts_per_frequency("tweet3".into(), "minute".into()), vec![2, 1]);
    }
}
