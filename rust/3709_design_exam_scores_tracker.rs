/// LeetCode #3709 - Design Exam Scores Tracker
struct ExamTracker {
    times: Vec<i32>,
    prefix: Vec<i64>,
}

impl ExamTracker {
    fn new() -> Self {
        Self {
            times: Vec::new(),
            prefix: vec![0],
        }
    }

    fn record(&mut self, time: i32, score: i32) {
        self.times.push(time);
        let last = *self.prefix.last().unwrap();
        self.prefix.push(last + score as i64);
    }

    fn total_score(&self, start_time: i32, end_time: i32) -> i64 {
        let i = self.times.partition_point(|&t| t < start_time);
        let j = self.times.partition_point(|&t| t <= end_time);
        if i >= j {
            return 0;
        }
        self.prefix[j] - self.prefix[i]
    }
}

fn main() {
    let mut et = ExamTracker::new();
    et.record(1, 98);
    println!("{}", et.total_score(1, 1));
}

#[cfg(test)]
mod tests {
    use super::ExamTracker;

    #[test]
    fn example1() {
        let mut et = ExamTracker::new();
        et.record(1, 98);
        assert_eq!(et.total_score(1, 1), 98);
        et.record(5, 99);
        assert_eq!(et.total_score(1, 3), 98);
        assert_eq!(et.total_score(1, 5), 197);
        assert_eq!(et.total_score(3, 4), 0);
        assert_eq!(et.total_score(2, 5), 99);
    }
}
