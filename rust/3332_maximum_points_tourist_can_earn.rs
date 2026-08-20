/// LeetCode #3332 - Maximum Points Tourist Can Earn
fn max_score(n: i32, k: i32, stay_score: Vec<Vec<i32>>, travel_score: Vec<Vec<i32>>) -> i32 {
    let n = n as usize;
    let k = k as usize;
    let neg = i32::MIN / 4;
    let mut f = vec![vec![neg; n]; k + 1];
    for j in 0..n {
        f[0][j] = 0;
    }
    for i in 1..=k {
        for j in 0..n {
            for h in 0..n {
                let add = if j == h {
                    stay_score[i - 1][j]
                } else {
                    travel_score[h][j]
                };
                f[i][j] = f[i][j].max(f[i - 1][h] + add);
            }
        }
    }
    *f[k].iter().max().unwrap()
}

fn main() {
    println!(
        "{}",
        max_score(2, 1, vec![vec![2, 3]], vec![vec![0, 2], vec![1, 0]])
    );
}

#[cfg(test)]
mod tests {
    use super::max_score;

    #[test]
    fn example1() {
        assert_eq!(
            max_score(2, 1, vec![vec![2, 3]], vec![vec![0, 2], vec![1, 0]]),
            3
        );
    }

    #[test]
    fn example2() {
        assert_eq!(
            max_score(
                3,
                2,
                vec![vec![3, 4, 2], vec![2, 1, 2]],
                vec![vec![0, 2, 1], vec![2, 0, 4], vec![3, 2, 0]]
            ),
            8
        );
    }
}
