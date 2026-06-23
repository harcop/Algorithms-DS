/// LeetCode #2055 - Plates Between Candles
fn plates_between_candles(s: String, queries: Vec<Vec<i32>>) -> Vec<i32> {
    let s = s.as_bytes();
    let n = s.len();
    let mut prefix = vec![0i32; n + 1];
    for i in 0..n {
        prefix[i + 1] = prefix[i] + (s[i] == b'*') as i32;
    }

    let mut left = vec![-1i32; n];
    let mut right = vec![-1i32; n];
    let mut l = -1i32;
    for i in 0..n {
        if s[i] == b'|' {
            l = i as i32;
        }
        left[i] = l;
    }
    let mut r = -1i32;
    for i in (0..n).rev() {
        if s[i] == b'|' {
            r = i as i32;
        }
        right[i] = r;
    }

    queries
        .iter()
        .map(|q| {
            let li = q[0] as usize;
            let ri = q[1] as usize;
            let i = right[li] as usize;
            let j = left[ri] as usize;
            if i < j {
                prefix[j] - prefix[i + 1]
            } else {
                0
            }
        })
        .collect()
}

fn main() {
    println!(
        "{:?}",
        plates_between_candles("**|**|***|".into(), vec![vec![2, 5], vec![5, 9]])
    );
}

#[cfg(test)]
mod tests {
    use super::plates_between_candles;

    #[test]
    fn example_one() {
        assert_eq!(
            plates_between_candles("**|**|***|".into(), vec![vec![2, 5], vec![5, 9]]),
            vec![2, 3]
        );
    }

    #[test]
    fn example_two() {
        assert_eq!(
            plates_between_candles(
                "***|**|*****|**||**|*".into(),
                vec![
                    vec![1, 17],
                    vec![4, 5],
                    vec![14, 17],
                    vec![5, 11],
                    vec![15, 16],
                ],
            ),
            vec![9, 0, 0, 0, 0]
        );
    }
}
