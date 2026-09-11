/// LeetCode #3693 - Climbing Stairs II
fn climb_stairs(n: i32, costs: Vec<i32>) -> i32 {
    let n = costs.len();
    let mut f = vec![i32::MAX / 4; n + 1];
    f[0] = 0;
    for (i, &x) in costs.iter().enumerate() {
        let i = i + 1;
        for j in i.saturating_sub(3)..i {
            let step = (i - j) as i32;
            f[i] = f[i].min(f[j] + x + step * step);
        }
    }
    f[n]
}

fn main() {
    println!("{}", climb_stairs(4, vec![1, 2, 3, 4]));
}

#[cfg(test)]
mod tests {
    use super::climb_stairs;

    #[test]
    fn example1() {
        assert_eq!(climb_stairs(4, vec![1, 2, 3, 4]), 13);
    }

    #[test]
    fn example2() {
        assert_eq!(climb_stairs(4, vec![5, 1, 6, 2]), 11);
    }

    #[test]
    fn example3() {
        assert_eq!(climb_stairs(3, vec![9, 8, 3]), 12);
    }
}
