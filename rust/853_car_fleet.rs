/// LeetCode #853 - Car Fleet
fn car_fleet(target: i32, position: Vec<i32>, speed: Vec<i32>) -> i32 {
    let mut cars: Vec<(i32, i32)> = position.into_iter().zip(speed).collect();
    cars.sort_by(|a, b| b.0.cmp(&a.0));
    let mut fleets = 0;
    let mut prev_time = 0.0;
    for (pos, sp) in cars {
        let time = (target - pos) as f64 / sp as f64;
        if time > prev_time {
            fleets += 1;
            prev_time = time;
        }
    }
    fleets
}

fn main() {
    println!("{}", car_fleet(12, vec![10, 8, 0, 5, 3], vec![2, 4, 1, 1, 3]));
}

#[cfg(test)]
mod tests {
    use super::car_fleet;

    #[test]
    fn example_one() {
        assert_eq!(car_fleet(12, vec![10, 8, 0, 5, 3], vec![2, 4, 1, 1, 3]), 3);
    }
}
