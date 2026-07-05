/// LeetCode #2237 - Count Positions on Street With Required Brightness
fn meet_requirement(n: i32, lights: Vec<Vec<i32>>, requirement: Vec<i32>) -> i32 {
    let n = n as usize;
    let mut change = vec![0i32; n + 1];
    for light in lights {
        let position = light[0] as usize;
        let range = light[1] as usize;
        change[position.saturating_sub(range)] += 1;
        if position + range + 1 <= n {
            change[position + range + 1] -= 1;
        }
    }

    let mut ans = 0i32;
    let mut brightness = 0i32;
    for i in 0..n {
        brightness += change[i];
        if brightness >= requirement[i] {
            ans += 1;
        }
    }
    ans
}

fn main() {
    println!(
        "{}",
        meet_requirement(5, vec![vec![0, 1], vec![2, 1], vec![3, 2]], vec![0, 2, 1, 4, 1])
    );
}

#[cfg(test)]
mod tests {
    use super::meet_requirement;

    #[test]
    fn example_one() {
        assert_eq!(
            meet_requirement(5, vec![vec![0, 1], vec![2, 1], vec![3, 2]], vec![0, 2, 1, 4, 1]),
            4
        );
    }

    #[test]
    fn example_two() {
        assert_eq!(meet_requirement(1, vec![vec![0, 1]], vec![2]), 0);
    }
}
