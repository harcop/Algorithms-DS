/// LeetCode #355 - Design Twitter
use std::collections::{HashMap, HashSet, BinaryHeap};

struct Twitter {
    time: i32,
    tweets: HashMap<i32, Vec<(i32,i32)>>,
    follows: HashMap<i32, HashSet<i32>>,
}

impl Twitter {
    fn new() -> Self { Self { time: 0, tweets: HashMap::new(), follows: HashMap::new() } }
    fn post_tweet(&mut self, user_id: i32, tweet_id: i32) {
        self.time += 1;
        self.tweets.entry(user_id).or_default().push((self.time, tweet_id));
    }
    fn get_news_feed(&mut self, user_id: i32) -> Vec<i32> {
        let mut users = self.follows.get(&user_id).cloned().unwrap_or_default();
        users.insert(user_id);
        let mut h = BinaryHeap::new();
        for u in users {
            if let Some(v) = self.tweets.get(&u) {
                for &(t,id) in v.iter().rev().take(10) {
                    h.push((t,id));
                }
            }
        }
        let mut out = vec![];
        for _ in 0..10 {
            if let Some((_,id)) = h.pop() { out.push(id); } else { break; }
        }
        out
    }
    fn follow(&mut self, follower_id: i32, followee_id: i32) {
        if follower_id != followee_id {
            self.follows.entry(follower_id).or_default().insert(followee_id);
        }
    }
    fn unfollow(&mut self, follower_id: i32, followee_id: i32) {
        if let Some(s) = self.follows.get_mut(&follower_id) { s.remove(&followee_id); }
    }
}

fn main() {
    let mut t = Twitter::new();
    t.post_tweet(1,5);
    println!("{:?}", t.get_news_feed(1));
}

#[cfg(test)]
mod tests {
    use super::Twitter;

    #[test]
    fn example() {
        let mut t = Twitter::new();
        t.post_tweet(1,5);
        assert_eq!(t.get_news_feed(1), vec![5]);
        t.follow(1,2);
        t.post_tweet(2,6);
        assert_eq!(t.get_news_feed(1), vec![6,5]);
        t.unfollow(1,2);
        assert_eq!(t.get_news_feed(1), vec![5]);
    }
}
