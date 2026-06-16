/// LeetCode #1927 - Sum Game
fn sum_game(num: String) -> bool {
    let bytes = num.as_bytes();
    let n = bytes.len();
    let half = n / 2;
    let mut cnt1 = 0i32;
    let mut cnt2 = 0i32;
    let mut s1 = 0i64;
    let mut s2 = 0i64;
    for i in 0..half {
        if bytes[i] == b'?' {
            cnt1 += 1;
        } else {
            s1 += (bytes[i] - b'0') as i64;
        }
    }
    for i in half..n {
        if bytes[i] == b'?' {
            cnt2 += 1;
        } else {
            s2 += (bytes[i] - b'0') as i64;
        }
    }
    (cnt1 + cnt2) % 2 == 1 || s1 - s2 != 9 * (cnt2 - cnt1) as i64 / 2
}

fn main() {
    println!("{}", sum_game("5023".into()));
}

#[cfg(test)]
mod tests {
    use super::sum_game;

    #[test]
    fn example_one() {
        assert!(!sum_game("5023".into()));
    }

    #[test]
    fn example_two() {
        assert!(sum_game("25??".into()));
    }

    #[test]
    fn example_three() {
        assert!(sum_game("?329".into()));
    }
}
