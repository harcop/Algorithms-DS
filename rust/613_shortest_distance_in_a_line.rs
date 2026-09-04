/// LeetCode #613 - Shortest Distance in a Line (SQL; Rust analogue)
fn shortest_distance_line(mut point: Vec<i32>) -> i32 {
    point.sort();
    point.windows(2).map(|w| w[1] - w[0]).min().unwrap()
}

fn main() {
    println!("ok");
}

#[cfg(test)]
mod tests {
    use super::shortest_distance_line;

    #[test]
    fn example() {
        assert_eq!(shortest_distance_line(vec![-1, 0, 2]), 1);
    }
}
