/// LeetCode #874 - Walking Robot Simulation
use std::collections::HashSet;

fn robot_sim(commands: Vec<i32>, obstacles: Vec<Vec<i32>>) -> i32 {
    let obs: HashSet<(i32, i32)> = obstacles.into_iter().map(|v| (v[0], v[1])).collect();
    let mut x = 0i32;
    let mut y = 0i32;
    let mut dir = 0usize; // 0 N, 1 E, 2 S, 3 W
    let dx = [0, 1, 0, -1];
    let dy = [1, 0, -1, 0];
    let mut best = 0;

    for c in commands {
        match c {
            -2 => dir = (dir + 3) % 4,
            -1 => dir = (dir + 1) % 4,
            k if k >= 1 && k <= 9 => {
                for _ in 0..k {
                    let nx = x + dx[dir];
                    let ny = y + dy[dir];
                    if obs.contains(&(nx, ny)) {
                        break;
                    }
                    x = nx;
                    y = ny;
                    best = best.max(x * x + y * y);
                }
            }
            _ => {}
        }
    }
    best
}

fn main() {
    println!("{}", robot_sim(vec![4, -1, 3], vec![]));
}

#[cfg(test)]
mod tests {
    use super::robot_sim;

    #[test]
    fn example_one() {
        assert_eq!(robot_sim(vec![4, -1, 3], vec![]), 25);
    }
}
