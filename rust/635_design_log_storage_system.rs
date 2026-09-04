/// LeetCode #635 - Design Log Storage System
struct LogSystem {
    logs: Vec<(i32, String)>,
}

impl LogSystem {
    fn new() -> Self {
        LogSystem { logs: vec![] }
    }

    fn put(&mut self, id: i32, timestamp: String) {
        self.logs.push((id, timestamp));
    }

    fn retrieve(&self, start: String, end: String, granularity: String) -> Vec<i32> {
        let cut = match granularity.as_str() {
            "Year" => 4,
            "Month" => 7,
            "Day" => 10,
            "Hour" => 13,
            "Minute" => 16,
            _ => 19,
        };
        let s = &start[..cut];
        let e = &end[..cut];
        self.logs
            .iter()
            .filter(|(_, ts)| {
                let t = &ts[..cut];
                t >= s && t <= e
            })
            .map(|(id, _)| *id)
            .collect()
    }
}

fn main() {
    let mut ls = LogSystem::new();
    ls.put(1, "2017:01:01:23:59:59".into());
    println!("{:?}", ls.retrieve("2016:01:01:00:00:00".into(), "2017:01:01:23:59:59".into(), "Year".into()));
}

#[cfg(test)]
mod tests {
    use super::LogSystem;

    #[test]
    fn example() {
        let mut ls = LogSystem::new();
        ls.put(1, "2017:01:01:23:59:59".into());
        ls.put(2, "2017:01:01:22:59:59".into());
        ls.put(3, "2016:01:01:00:00:00".into());
        assert_eq!(
            ls.retrieve(
                "2016:01:01:01:01:01".into(),
                "2017:01:01:23:00:00".into(),
                "Year".into()
            ),
            vec![1, 2, 3]
        );
        assert_eq!(
            ls.retrieve(
                "2016:01:01:01:01:01".into(),
                "2017:01:01:23:00:00".into(),
                "Hour".into()
            ),
            vec![1, 2]
        );
    }
}
