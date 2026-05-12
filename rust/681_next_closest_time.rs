/// LeetCode #681 - Next Closest Time
fn next_closest_time(time: String) -> String {
    let digits: Vec<u32> = time
        .chars()
        .filter(|c| c.is_ascii_digit())
        .map(|c| c.to_digit(10).unwrap())
        .collect();
    let parts: Vec<&str> = time.split(':').collect();
    let cur = parts[0].parse::<i32>().unwrap() * 60 + parts[1].parse::<i32>().unwrap();
    let mut best: Option<(i32, i32)> = None;
    let mut best_diff = i32::MAX;
    for &a in &digits {
        for &b in &digits {
            for &c in &digits {
                for &d in &digits {
                    let h = (a * 10 + b) as i32;
                    let m = (c * 10 + d) as i32;
                    if h >= 24 || m >= 60 {
                        continue;
                    }
                    let t = h * 60 + m;
                    let diff = (t - cur).rem_euclid(24 * 60);
                    if diff > 0 && diff < best_diff {
                        best_diff = diff;
                        best = Some((h, m));
                    }
                }
            }
        }
    }
    match best {
        Some((h, m)) => format!("{:02}:{:02}", h, m),
        None => time,
    }
}

fn main() {
    println!("{}", next_closest_time("19:34".into()));
}

#[cfg(test)]
mod tests {
    use super::next_closest_time;

    #[test]
    fn example_one() {
        assert_eq!(next_closest_time("19:34".into()), "19:39");
    }

    #[test]
    fn example_two() {
        assert_eq!(next_closest_time("23:59".into()), "22:22");
    }
}
