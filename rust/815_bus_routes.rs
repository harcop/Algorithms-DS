/// LeetCode #815 - Bus Routes
use std::collections::{HashMap, HashSet, VecDeque};

fn num_buses_to_destination(routes: Vec<Vec<i32>>, source: i32, target: i32) -> i32 {
    if source == target {
        return 0;
    }
    let mut stop_to_routes: HashMap<i32, Vec<usize>> = HashMap::new();
    for (i, r) in routes.iter().enumerate() {
        for &s in r {
            stop_to_routes.entry(s).or_default().push(i);
        }
    }
    let mut q = VecDeque::new();
    let mut seen_routes = HashSet::new();
    let mut seen_stops = HashSet::new();
    q.push_back((source, 1));
    seen_stops.insert(source);
    while let Some((stop, buses)) = q.pop_front() {
        for &ri in stop_to_routes.get(&stop).unwrap_or(&vec![]) {
            if seen_routes.contains(&ri) {
                continue;
            }
            seen_routes.insert(ri);
            for &next in &routes[ri] {
                if next == target {
                    return buses;
                }
                if seen_stops.insert(next) {
                    q.push_back((next, buses + 1));
                }
            }
        }
    }
    -1
}

fn main() {
    let r = vec![vec![1, 2, 7], vec![3, 6, 7]];
    println!("{}", num_buses_to_destination(r, 1, 6));
}

#[cfg(test)]
mod tests {
    use super::num_buses_to_destination;

    #[test]
    fn example_one() {
        let r = vec![vec![1, 2, 7], vec![3, 6, 7]];
        assert_eq!(num_buses_to_destination(r, 1, 6), 2);
    }
}
