/// LeetCode #2594 - Minimum Time to Repair Cars
fn repair_cars(ranks: Vec<i32>, cars: i32) -> i64 {
    let cars = cars as i64;
    let mut left = 0i64;
    let mut right = ranks[0] as i64 * cars * cars;
    while left < right {
        let mid = (left + right) / 2;
        let mut cnt = 0i64;
        for &r in &ranks {
            cnt += ((mid / r as i64) as f64).sqrt() as i64;
        }
        if cnt >= cars {
            right = mid;
        } else {
            left = mid + 1;
        }
    }
    left
}

fn main() {
    println!("{}", repair_cars(vec![4, 2, 3, 1], 10));
}

#[cfg(test)]
mod tests {
    use super::repair_cars;

    #[test]
    fn example_one() {
        assert_eq!(repair_cars(vec![4, 2, 3, 1], 10), 16);
    }

    #[test]
    fn example_two() {
        assert_eq!(repair_cars(vec![5, 1, 8], 6), 16);
    }
}
