/// LeetCode #539 - Minimum Time Difference
fn find_min_difference(time_points: Vec<String>) -> i32 {
    let mut mins: Vec<i32> = time_points
        .into_iter()
        .map(|t| {
            let (h, m) = t.split_once(':').unwrap();
            h.parse::<i32>().unwrap() * 60 + m.parse::<i32>().unwrap()
        })
        .collect();
    mins.sort_unstable();
    let mut best = i32::MAX;
    for i in 0..mins.len() - 1 {
        best = best.min(mins[i + 1] - mins[i]);
    }
    let wrap = 24 * 60 - mins.last().unwrap() + mins[0];
    best.min(wrap)
}

fn main() {
    println!("{}", find_min_difference(vec!["23:59".into(), "00:00".into()]));
}

#[cfg(test)]
mod tests {
    use super::find_min_difference;

    #[test]
    fn example_one() {
        assert_eq!(find_min_difference(vec!["23:59".into(), "00:00".into()]), 1);
    }
}
