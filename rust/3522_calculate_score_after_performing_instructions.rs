/// LeetCode #3522 - Calculate Score After Performing Instructions
fn calculate_score(instructions: Vec<String>, values: Vec<i32>) -> i64 {
    let n = values.len();
    let mut vis = vec![false; n];
    let mut ans = 0i64;
    let mut i: i64 = 0;
    while i >= 0 && (i as usize) < n && !vis[i as usize] {
        let idx = i as usize;
        vis[idx] = true;
        if instructions[idx].starts_with('a') {
            ans += values[idx] as i64;
            i += 1;
        } else {
            i += values[idx] as i64;
        }
    }
    ans
}

fn main() {
    println!(
        "{}",
        calculate_score(
            vec!["jump".into(), "add".into(), "add".into(), "jump".into(), "add".into(), "jump".into()],
            vec![2, 1, 3, 1, -2, -3]
        )
    );
}

#[cfg(test)]
mod tests {
    use super::calculate_score;

    #[test]
    fn example1() {
        assert_eq!(
            calculate_score(
                vec![
                    "jump".into(),
                    "add".into(),
                    "add".into(),
                    "jump".into(),
                    "add".into(),
                    "jump".into()
                ],
                vec![2, 1, 3, 1, -2, -3]
            ),
            1
        );
    }

    #[test]
    fn example2() {
        assert_eq!(
            calculate_score(vec!["jump".into(), "add".into(), "add".into()], vec![3, 1, 1]),
            0
        );
    }

    #[test]
    fn example3() {
        assert_eq!(calculate_score(vec!["jump".into()], vec![0]), 0);
    }
}
