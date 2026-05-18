/// LeetCode #871 - Minimum Number of Refueling Stops
use std::collections::BinaryHeap;

fn min_refuel_stops(target: i32, start_fuel: i32, stations: Vec<Vec<i32>>) -> i32 {
    let mut fuel = start_fuel as i64;
    let target = target as i64;
    let mut heap = BinaryHeap::new();
    let mut prev = 0i64;
    let mut ans = 0;
    let n = stations.len();
    let mut i = 0;

    loop {
        let next_pos = if i < n { stations[i][0] as i64 } else { target };
        let dist = next_pos - prev;
        while fuel < dist {
            if let Some(f) = heap.pop() {
                fuel += f as i64;
                ans += 1;
            } else {
                return -1;
            }
        }
        fuel -= dist;
        prev = next_pos;
        if i < n {
            heap.push(stations[i][1]);
            i += 1;
        } else {
            break;
        }
    }
    ans
}

fn main() {
    println!("{}", min_refuel_stops(100, 10, vec![vec![10, 60], vec![20, 30], vec![30, 30], vec![60, 40]]));
}

#[cfg(test)]
mod tests {
    use super::min_refuel_stops;

    #[test]
    fn example_one() {
        assert_eq!(
            min_refuel_stops(100, 10, vec![vec![10, 60], vec![20, 30], vec![30, 30], vec![60, 40]]),
            2
        );
    }
}
