/// LeetCode #3998 - Transform Binary String Using Subsequence Sort
fn can_transform(s: String, strs: Vec<String>) -> Vec<bool> {
    let s = s.as_bytes();
    let n = s.len();
    let mut pref = vec![0i32; n + 1];
    for i in 0..n {
        pref[i + 1] = pref[i] + i32::from(s[i] == b'1');
    }
    let total = pref[n];
    strs.into_iter()
        .map(|p| {
            let p = p.as_bytes();
            let fixed = p.iter().filter(|&&c| c == b'1').count() as i32;
            let questions = p.iter().filter(|&&c| c == b'?').count() as i32;
            let need = total - fixed;
            if need < 0 || need > questions {
                return false;
            }
            let mut ones = 0i32;
            let mut rem_need = need;
            let mut rem_q = questions;
            for i in 0..n {
                if p[i] == b'1' {
                    ones += 1;
                } else if p[i] == b'?' {
                    rem_q -= 1;
                    if rem_q < rem_need {
                        ones += 1;
                        rem_need -= 1;
                    }
                }
                if ones > pref[i + 1] {
                    return false;
                }
            }
            rem_need == 0
        })
        .collect()
}

fn main() {
    println!(
        "{:?}",
        can_transform("101".into(), vec!["1?1".into(), "0?1".into(), "0?0".into()])
    );
}

#[cfg(test)]
mod tests {
    use super::can_transform;

    #[test]
    fn example1() {
        assert_eq!(
            can_transform("101".into(), vec!["1?1".into(), "0?1".into(), "0?0".into()]),
            vec![true, true, false]
        );
    }

    #[test]
    fn example2() {
        assert_eq!(
            can_transform(
                "1100".into(),
                vec!["0011".into(), "11?1".into(), "1?1?".into()]
            ),
            vec![true, false, true]
        );
    }

    #[test]
    fn example3() {
        assert_eq!(
            can_transform("1010".into(), vec!["0011".into()]),
            vec![true]
        );
    }
}
