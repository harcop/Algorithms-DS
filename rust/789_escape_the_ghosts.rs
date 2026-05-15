/// LeetCode #789 - Escape The Ghosts
fn escape_ghosts(ghosts: Vec<Vec<i32>>, target: Vec<i32>) -> bool {
    let mut d = 0i32;
    for g in &ghosts {
        d = d.max((g[0] - target[0]).abs() + (g[1] - target[1]).abs());
    }
    d > (target[0].abs() + target[1].abs())
}

fn main() {
    println!("{}", escape_ghosts(vec![vec![1, 0], vec![0, 3]], vec![0, 1]));
}

#[cfg(test)]
mod tests {
    use super::escape_ghosts;

    #[test]
    fn example_one() {
        assert!(escape_ghosts(vec![vec![1, 0], vec![0, 3]], vec![0, 1]));
    }

    #[test]
    fn example_two() {
        assert!(!escape_ghosts(vec![vec![1, 0]], vec![2, 0]));
    }
}
