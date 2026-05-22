/// LeetCode #1184 - Distance Between Bus Stops
fn distance_between_bus_stops(distance: Vec<i32>, start: i32, destination: i32) -> i32 {
    let n = distance.len();
    let mut a = start as usize;
    let mut b = destination as usize;
    if a > b {
        std::mem::swap(&mut a, &mut b);
    }
    let mut cw = 0i32;
    for i in a..b {
        cw += distance[i];
    }
    let mut ccw = 0i32;
    for i in 0..a {
        ccw += distance[i];
    }
    for i in b..n {
        ccw += distance[i];
    }
    cw.min(ccw)
}

fn main() {
    println!("{}", distance_between_bus_stops(vec![1, 2, 3, 4], 0, 1));
}

#[cfg(test)]
mod tests {
    use super::distance_between_bus_stops;

    #[test]
    fn example_one() {
        assert_eq!(distance_between_bus_stops(vec![1, 2, 3, 4], 0, 3), 4);
    }

    #[test]
    fn example_two() {
        assert_eq!(distance_between_bus_stops(vec![1, 2, 3, 4], 0, 1), 1);
    }
}
