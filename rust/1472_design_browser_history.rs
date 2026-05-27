/// LeetCode #1472 - Design Browser History
pub struct BrowserHistory {
    history: Vec<String>,
    idx: usize,
}
impl BrowserHistory {
    fn new(homepage: String) -> Self {
        BrowserHistory { history: vec![homepage], idx: 0 }
    }
    fn visit(&mut self, url: String) {
        self.history.truncate(self.idx + 1);
        self.history.push(url);
        self.idx = self.history.len() - 1;
    }
    fn back(&mut self, steps: i32) -> String {
        self.idx = self.idx.saturating_sub(steps as usize);
        self.history[self.idx].clone()
    }
    fn forward(&mut self, steps: i32) -> String {
        let max_idx = self.idx + steps as usize;
        self.idx = max_idx.min(self.history.len() - 1);
        self.history[self.idx].clone()
    }
}
fn main() {
    let mut b = BrowserHistory::new("leetcode.com".into());
    b.visit("google.com".into());
    println!("{}", b.back(1));
}
#[cfg(test)]
mod tests {
    use super::BrowserHistory;
    #[test]
    fn example_one() {
        let mut b = BrowserHistory::new("leetcode.com".into());
        b.visit("google.com".into());
        b.visit("facebook.com".into());
        b.visit("youtube.com".into());
        assert_eq!(b.back(1), "facebook.com");
        assert_eq!(b.back(1), "google.com");
        assert_eq!(b.forward(1), "facebook.com");
        b.visit("linkedin.com".into());
        assert_eq!(b.forward(2), "linkedin.com");
        assert_eq!(b.back(2), "google.com");
        assert_eq!(b.back(7), "leetcode.com");
        assert_eq!(b.forward(1), "google.com");
    }
}