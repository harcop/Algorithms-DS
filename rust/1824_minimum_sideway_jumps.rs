/// LeetCode #1824 - Minimum Sideway Jumps
fn min_side_jumps(obstacles: Vec<i32>) -> i32 {
    const INF: i32 = i32::MAX / 4;
    let mut f = [1, 0, 1];
    for &v in obstacles.iter().skip(1) {
        for j in 0..3 {
            if v == (j + 1) as i32 {
                f[j] = INF;
                break;
            }
        }
        let x = f.iter().copied().min().unwrap() + 1;
        for j in 0..3 {
            if v != (j + 1) as i32 {
                f[j] = f[j].min(x);
            }
        }
    }
    *f.iter().min().unwrap()
}

fn main() {
    println!("{}", min_side_jumps(vec![0, 1, 2, 3, 0]));
}

#[cfg(test)]
mod tests {
    use super::min_side_jumps;

    #[test]
    fn example_one() {
        assert_eq!(min_side_jumps(vec![0, 1, 2, 3, 0]), 2);
    }

    #[test]
    fn example_two() {
        assert_eq!(min_side_jumps(vec![0, 1, 1, 3, 3, 0]), 0);
    }
}
