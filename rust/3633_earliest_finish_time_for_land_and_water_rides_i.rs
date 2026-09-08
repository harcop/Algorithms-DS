/// LeetCode #3633 - Earliest Finish Time for Land and Water Rides I
fn calc(a1: &[i32], t1: &[i32], a2: &[i32], t2: &[i32]) -> i32 {
    let min_end = a1.iter().zip(t1.iter()).map(|(a, t)| a + t).min().unwrap();
    a2.iter()
        .zip(t2.iter())
        .map(|(a, t)| a.max(&min_end) + t)
        .min()
        .unwrap()
}

fn earliest_finish_time(
    land_start_time: Vec<i32>,
    land_duration: Vec<i32>,
    water_start_time: Vec<i32>,
    water_duration: Vec<i32>,
) -> i32 {
    let x = calc(
        &land_start_time,
        &land_duration,
        &water_start_time,
        &water_duration,
    );
    let y = calc(
        &water_start_time,
        &water_duration,
        &land_start_time,
        &land_duration,
    );
    x.min(y)
}

fn main() {
    println!(
        "{}",
        earliest_finish_time(vec![2, 8], vec![4, 1], vec![6], vec![3])
    );
}

#[cfg(test)]
mod tests {
    use super::earliest_finish_time;

    #[test]
    fn example1() {
        assert_eq!(
            earliest_finish_time(vec![2, 8], vec![4, 1], vec![6], vec![3]),
            9
        );
    }

    #[test]
    fn example2() {
        assert_eq!(
            earliest_finish_time(vec![5], vec![3], vec![1], vec![10]),
            14
        );
    }
}
