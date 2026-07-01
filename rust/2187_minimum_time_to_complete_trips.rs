/// LeetCode #2187 - Minimum Time to Complete Trips
fn minimum_time(time: Vec<i32>, total_trips: i64) -> i64 {
    let min_time = *time.iter().min().unwrap() as i64;
    let mut left = 1i64;
    let mut right = min_time * total_trips;

    while left < right {
        let mid = left + (right - left) / 2;
        let trips: i64 = time.iter().map(|&t| mid / t as i64).sum();
        if trips >= total_trips {
            right = mid;
        } else {
            left = mid + 1;
        }
    }

    left
}

fn main() {
    println!("{}", minimum_time(vec![1, 2, 3], 5));
}

#[cfg(test)]
mod tests {
    use super::minimum_time;

    #[test]
    fn example_one() {
        assert_eq!(minimum_time(vec![1, 2, 3], 5), 3);
    }

    #[test]
    fn example_two() {
        assert_eq!(minimum_time(vec![2], 1), 2);
    }
}
