/// LeetCode #1242 - Web Crawler Multithreaded (sequential analogue)
use std::collections::{HashMap, HashSet, VecDeque};

struct HtmlParser {
    links: HashMap<String, Vec<String>>,
}

impl HtmlParser {
    fn new(links: HashMap<String, Vec<String>>) -> Self {
        HtmlParser { links }
    }

    fn get_urls(&self, url: &str) -> Vec<String> {
        self.links.get(url).cloned().unwrap_or_default()
    }
}

fn hostname(url: &str) -> &str {
    let rest = url
        .strip_prefix("http://")
        .or_else(|| url.strip_prefix("https://"))
        .unwrap_or(url);
    rest.split('/').next().unwrap_or(rest)
}

fn crawl(start_url: String, html_parser: &HtmlParser) -> Vec<String> {
    let host = hostname(&start_url).to_string();
    let mut seen = HashSet::new();
    let mut q = VecDeque::new();
    seen.insert(start_url.clone());
    q.push_back(start_url);
    let mut out = Vec::new();
    while let Some(url) = q.pop_front() {
        out.push(url.clone());
        for nxt in html_parser.get_urls(&url) {
            if hostname(&nxt) == host && seen.insert(nxt.clone()) {
                q.push_back(nxt);
            }
        }
    }
    out
}

fn main() {
    let parser = HtmlParser::new(HashMap::new());
    println!("{:?}", crawl("http://news.yahoo.com".into(), &parser));
}

#[cfg(test)]
mod tests {
    use super::{crawl, HtmlParser};
    use std::collections::HashMap;

    #[test]
    fn example() {
        let mut links = HashMap::new();
        links.insert(
            "http://news.yahoo.com".into(),
            vec!["http://news.yahoo.com/news".into(), "http://news.yahoo.com/us".into()],
        );
        links.insert(
            "http://news.yahoo.com/news".into(),
            vec![
                "http://news.yahoo.com/news/topics/".into(),
                "http://news.yahoo.com".into(),
                "http://news.yahoo.com/us".into(),
            ],
        );
        links.insert(
            "http://news.yahoo.com/news/topics/".into(),
            vec![
                "http://news.yahoo.com".into(),
                "http://news.yahoo.com/news".into(),
                "http://news.google.com".into(),
            ],
        );
        links.insert("http://news.yahoo.com/us".into(), vec![]);
        links.insert("http://news.google.com".into(), vec![]);
        let parser = HtmlParser::new(links);
        let mut got = crawl("http://news.yahoo.com/news/topics/".into(), &parser);
        got.sort();
        let mut want = vec![
            "http://news.yahoo.com".to_string(),
            "http://news.yahoo.com/news".to_string(),
            "http://news.yahoo.com/news/topics/".to_string(),
            "http://news.yahoo.com/us".to_string(),
        ];
        want.sort();
        assert_eq!(got, want);
    }
}
