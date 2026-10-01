/// LeetCode #4024 - Nearest Available Drone
fn nearest_drone(drones: Vec<Vec<i32>>, target: Vec<i32>) -> i32 {
    let (tx, ty) = (target[0], target[1]);
    let mut ans = -1;
    let mut best = i32::MAX;
    for (i, d) in drones.iter().enumerate() {
        let dist = (d[0] - tx).abs() + (d[1] - ty).abs();
        if dist <= d[2] && dist < best {
            best = dist;
            ans = i as i32;
        }
    }
    ans
}

fn main() {
    println!("{}", nearest_drone(vec![vec![0, 0, 8], vec![2, 2, 9]], vec![3, 4]));
}

#[cfg(test)]
mod tests {
    use super::nearest_drone;

    #[test]
    fn example1() {
        assert_eq!(
            nearest_drone(vec![vec![0, 0, 8], vec![2, 2, 9]], vec![3, 4]),
            1
        );
    }

    #[test]
    fn example2() {
        assert_eq!(
            nearest_drone(vec![vec![2, 1, 5], vec![4, 4, 5], vec![6, 6, 8]], vec![5, 5]),
            1
        );
    }

    #[test]
    fn example3() {
        assert_eq!(nearest_drone(vec![vec![4, 4, 5]], vec![8, 6]), -1);
    }
}
