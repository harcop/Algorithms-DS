/// LeetCode #3664 - Two-Letter Card Game
fn score(cards: Vec<String>, x: char) -> i32 {
    let mut cnt1 = [0i32; 10];
    let mut cnt2 = [0i32; 10];
    let mut both = 0i32;
    for s in &cards {
        let b = s.as_bytes();
        let a = b[0] as char;
        let c = b[1] as char;
        if a == x && c == x {
            both += 1;
        } else if a == x {
            cnt1[(c as u8 - b'a') as usize] += 1;
        } else if c == x {
            cnt2[(a as u8 - b'a') as usize] += 1;
        }
    }
    let solve = |cnt: &[i32; 10], have: i32| -> i32 {
        let mut total = have;
        let mut mx = have;
        for &v in cnt {
            total += v;
            mx = mx.max(v);
        }
        (total / 2).min(total - mx)
    };
    let mut ans = 0;
    for give in 0..=both {
        ans = ans.max(solve(&cnt1, give) + solve(&cnt2, both - give));
    }
    ans
}

fn main() {
    println!(
        "{}",
        score(
            vec!["aa", "ab", "ba", "ac"]
                .into_iter()
                .map(String::from)
                .collect(),
            'a'
        )
    );
}

#[cfg(test)]
mod tests {
    use super::score;

    #[test]
    fn example1() {
        assert_eq!(
            score(
                vec!["aa", "ab", "ba", "ac"]
                    .into_iter()
                    .map(String::from)
                    .collect(),
                'a'
            ),
            2
        );
    }

    #[test]
    fn example2() {
        assert_eq!(
            score(
                vec!["aa", "ab", "ba"]
                    .into_iter()
                    .map(String::from)
                    .collect(),
                'a'
            ),
            1
        );
    }

    #[test]
    fn example3() {
        assert_eq!(
            score(
                vec!["aa", "ab", "ba", "ac"]
                    .into_iter()
                    .map(String::from)
                    .collect(),
                'b'
            ),
            0
        );
    }
}
