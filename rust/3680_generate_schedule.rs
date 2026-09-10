/// LeetCode #3680 - Generate Schedule
fn generate_schedule(n: i32) -> Vec<Vec<i32>> {
    let n = n as i32;
    let mut result: Vec<Vec<i32>> = Vec::new();
    if n <= 4 {
        return result;
    }
    let mut l = 1i32;
    if n % 2 == 0 {
        let mut i = 0;
        while i < n {
            result.push(vec![i, i + l]);
            i += 2;
        }
        i = 0;
        while i < n {
            result.push(vec![i + l, i]);
            i += 2;
        }
        i = 1;
        while i < n {
            result.push(vec![i, (i + l) % n]);
            i += 2;
        }
        i = 1;
        while i < n {
            result.push(vec![(i + l) % n, i]);
            i += 2;
        }
    } else {
        let mut i = 0;
        while i < 2 * n {
            result.push(vec![i % n, (i + l) % n]);
            i += 2;
        }
        i = 0;
        while i < 2 * n {
            result.push(vec![(i + l) % n, i % n]);
            i += 2;
        }
    }
    // Python: for l in range(2, (n+1)//2)
    l = 2;
    while l < (n + 1) / 2 {
        let j = result.last().unwrap()[0] + 1;
        for i in j..j + n {
            result.push(vec![((i % n) + n) % n, (((i + l) % n) + n) % n]);
        }
        let j = result.last().unwrap()[1] - 1;
        for i in j..j + n {
            result.push(vec![(((i + l) % n) + n) % n, ((i % n) + n) % n]);
        }
        l += 1;
    }
    if n % 2 == 0 {
        l = n / 2;
        let j = result.last().unwrap()[0] - 1;
        for i in j..j + n {
            result.push(vec![((i % n) + n) % n, (((i + l) % n) + n) % n]);
        }
    }
    result
}

fn valid_schedule(n: i32, schedule: &[Vec<i32>]) -> bool {
    if n <= 4 {
        return schedule.is_empty();
    }
    let expected = n * (n - 1);
    if schedule.len() as i32 != expected {
        return false;
    }
    use std::collections::HashSet;
    let mut seen = HashSet::new();
    for (day, match_) in schedule.iter().enumerate() {
        if match_.len() != 2 {
            return false;
        }
        let (a, b) = (match_[0], match_[1]);
        if a == b || a < 0 || b < 0 || a >= n || b >= n {
            return false;
        }
        if !seen.insert((a, b)) {
            return false;
        }
        if day > 0 {
            let prev = &schedule[day - 1];
            if a == prev[0] || a == prev[1] || b == prev[0] || b == prev[1] {
                return false;
            }
        }
    }
    true
}

fn main() {
    println!("{:?}", generate_schedule(3));
}

#[cfg(test)]
mod tests {
    use super::{generate_schedule, valid_schedule};

    #[test]
    fn example1() {
        assert!(generate_schedule(3).is_empty());
    }

    #[test]
    fn example2() {
        let s = generate_schedule(5);
        assert!(valid_schedule(5, &s));
    }

    #[test]
    fn n6() {
        let s = generate_schedule(6);
        assert!(valid_schedule(6, &s));
    }
}
