/// LeetCode #2381 - Shifting Letters II
fn shifting_letters(s: String, shifts: Vec<Vec<i32>>) -> String {
    let n = s.len();
    let mut d = vec![0i32; n + 1];

    for sh in shifts {
        let (start, end, mut dir) = (sh[0] as usize, sh[1] as usize, sh[2]);
        if dir == 0 {
            dir = -1;
        }
        d[start] += dir;
        d[end + 1] -= dir;
    }

    for i in 1..=n {
        d[i] += d[i - 1];
    }

    let bytes = s.into_bytes();
    let mut ans = Vec::with_capacity(n);
    for i in 0..n {
        let shifted = ((bytes[i] - b'a') as i32 + d[i].rem_euclid(26)) % 26;
        ans.push(b'a' + shifted as u8);
    }
    String::from_utf8(ans).unwrap()
}

fn main() {
    println!(
        "{}",
        shifting_letters(
            "abc".to_string(),
            vec![vec![0, 1, 0], vec![1, 2, 1], vec![0, 2, 1]]
        )
    );
}

#[cfg(test)]
mod tests {
    use super::shifting_letters;

    #[test]
    fn example_one() {
        assert_eq!(
            shifting_letters(
                "abc".to_string(),
                vec![vec![0, 1, 0], vec![1, 2, 1], vec![0, 2, 1]]
            ),
            "ace"
        );
    }

    #[test]
    fn example_two() {
        assert_eq!(
            shifting_letters("dztz".to_string(), vec![vec![0, 0, 0], vec![1, 1, 1]]),
            "catz"
        );
    }
}
