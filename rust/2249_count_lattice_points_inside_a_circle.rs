/// LeetCode #2249 - Count Lattice Points Inside a Circle
fn count_lattice_points(circles: Vec<Vec<i32>>) -> i32 {
    let mut ans = 0;
    for x in 0..=200 {
        for y in 0..=200 {
            if circles.iter().any(|c| {
                let dx = c[0] - x;
                let dy = c[1] - y;
                dx * dx + dy * dy <= c[2] * c[2]
            }) {
                ans += 1;
            }
        }
    }
    ans
}

fn main() {
    println!("{}", count_lattice_points(vec![vec![2, 2, 1]]));
}

#[cfg(test)]
mod tests {
    use super::count_lattice_points;

    #[test]
    fn example_one() {
        assert_eq!(count_lattice_points(vec![vec![2, 2, 1]]), 5);
    }

    #[test]
    fn example_two() {
        assert_eq!(
            count_lattice_points(vec![vec![2, 2, 2], vec![3, 4, 1]]),
            16
        );
    }
}
