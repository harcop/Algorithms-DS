/// LeetCode #759 - Employee Free Time
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Interval {
    pub start: i32,
    pub end: i32,
}

impl Interval {
    fn new(start: i32, end: i32) -> Self {
        Interval { start, end }
    }
}

fn employee_free_time(schedule: Vec<Vec<Interval>>) -> Vec<Interval> {
    let mut all: Vec<Interval> = schedule.into_iter().flatten().collect();
    all.sort_by_key(|iv| iv.start);
    let mut merged: Vec<Interval> = Vec::new();
    for iv in all {
        if let Some(last) = merged.last_mut() {
            if iv.start <= last.end {
                last.end = last.end.max(iv.end);
                continue;
            }
        }
        merged.push(iv);
    }
    let mut free = Vec::new();
    for w in merged.windows(2) {
        if w[0].end < w[1].start {
            free.push(Interval::new(w[0].end, w[1].start));
        }
    }
    free
}

fn main() {
    let schedule = vec![
        vec![Interval::new(1, 2), Interval::new(5, 6)],
        vec![Interval::new(1, 3)],
        vec![Interval::new(4, 10)],
    ];
    println!("{:?}", employee_free_time(schedule));
}

#[cfg(test)]
mod tests {
    use super::{employee_free_time, Interval};

    #[test]
    fn example_one() {
        let schedule = vec![
            vec![Interval::new(1, 2), Interval::new(5, 6)],
            vec![Interval::new(1, 3)],
            vec![Interval::new(4, 10)],
        ];
        assert_eq!(employee_free_time(schedule), vec![Interval::new(3, 4)]);
    }

    #[test]
    fn example_two() {
        let schedule = vec![
            vec![Interval::new(1, 3), Interval::new(6, 7)],
            vec![Interval::new(2, 4)],
            vec![Interval::new(2, 5), Interval::new(9, 12)],
        ];
        assert_eq!(
            employee_free_time(schedule),
            vec![Interval::new(5, 6), Interval::new(7, 9)]
        );
    }
}
