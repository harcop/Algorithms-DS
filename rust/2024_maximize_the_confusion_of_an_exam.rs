/// LeetCode #2024 - Maximize the Confusion of an Exam
fn max_consecutive_answers(answer_key: String, k: i32) -> i32 {
    let answer_key = answer_key.as_bytes();
    let k = k as i32;

    let f = |c: u8| -> i32 {
        let mut cnt = 0;
        let mut l = 0usize;
        for &ch in answer_key {
            if ch == c {
                cnt += 1;
            }
            if cnt > k {
                if answer_key[l] == c {
                    cnt -= 1;
                }
                l += 1;
            }
        }
        (answer_key.len() - l) as i32
    };

    f(b'T').max(f(b'F'))
}

fn main() {
    println!("{}", max_consecutive_answers("TTFF".into(), 2));
}

#[cfg(test)]
mod tests {
    use super::max_consecutive_answers;

    #[test]
    fn example_one() {
        assert_eq!(max_consecutive_answers("TTFF".into(), 2), 4);
    }

    #[test]
    fn example_two() {
        assert_eq!(max_consecutive_answers("TFFT".into(), 1), 3);
    }

    #[test]
    fn example_three() {
        assert_eq!(max_consecutive_answers("TTFTTFTT".into(), 1), 5);
    }
}
