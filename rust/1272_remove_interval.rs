/// LeetCode #1272 - Remove Interval
fn remove_interval(intervals: Vec<Vec<i32>>, to_be_removed: Vec<i32>) -> Vec<Vec<i32>> {
    let lo = to_be_removed[0];
    let hi = to_be_removed[1];
    let mut res = Vec::new();
    for iv in intervals {
        let a = iv[0];
        let b = iv[1];
        if b <= lo || a >= hi {
            res.push(iv);
        } else {
            if a < lo {
                res.push(vec![a, lo]);
            }
            if b > hi {
                res.push(vec![hi, b]);
            }
        }
    }
    res
}

fn main() {
    println!(
        "{:?}",
        remove_interval(
            vec![vec![0, 2], vec![3, 4], vec![5, 7], vec![8, 10], vec![11, 12]],
            vec![1, 3],
        )
    );
}

#[cfg(test)]
mod tests {
    use super::remove_interval;

    #[test]
    fn example_one() {
        assert_eq!(
            remove_interval(
                vec![vec![0, 2], vec![3, 4], vec![5, 7], vec![8, 10], vec![11, 12]],
                vec![1, 3],
            ),
            vec![vec![0, 1], vec![3, 4], vec![5, 7], vec![8, 10], vec![11, 12]]
        );
    }

    #[test]
    fn example_two() {
        assert_eq!(
            remove_interval(vec![vec![0, 5]], vec![2, 3]),
            vec![vec![0, 2], vec![3, 5]]
        );
    }

    #[test]
    fn example_three() {
        assert_eq!(
            remove_interval(
                vec![
                    vec![-1, 0],
                    vec![0, 5],
                    vec![8, 10],
                    vec![11, 12],
                    vec![15, 23],
                    vec![24, 25],
                ],
                vec![-2, -1],
            ),
            vec![
                vec![-1, 0],
                vec![0, 5],
                vec![8, 10],
                vec![11, 12],
                vec![15, 23],
                vec![24, 25],
            ]
        );
    }
}
