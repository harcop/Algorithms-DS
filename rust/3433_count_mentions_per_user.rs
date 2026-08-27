/// LeetCode #3433 - Count Mentions Per User
fn count_mentions(number_of_users: i32, mut events: Vec<Vec<String>>) -> Vec<i32> {
    events.sort_by(|a, b| {
        let ta = a[1].parse::<i32>().unwrap();
        let tb = b[1].parse::<i32>().unwrap();
        ta.cmp(&tb).then(a[0].as_bytes()[2].cmp(&b[0].as_bytes()[2]))
    });
    let n = number_of_users as usize;
    let mut ans = vec![0; n];
    let mut online_t = vec![0; n];
    let mut lazy = 0;
    for e in events {
        let cur: i32 = e[1].parse().unwrap();
        let etype = e[0].as_bytes()[0];
        let s = &e[2];
        if etype == b'O' {
            online_t[s.parse::<usize>().unwrap()] = cur + 60;
        } else if s.as_bytes()[0] == b'A' {
            lazy += 1;
        } else if s.as_bytes()[0] == b'H' {
            for i in 0..n {
                if online_t[i] <= cur {
                    ans[i] += 1;
                }
            }
        } else {
            for a in s.split_whitespace() {
                ans[a[2..].parse::<usize>().unwrap()] += 1;
            }
        }
    }
    if lazy > 0 {
        for x in &mut ans {
            *x += lazy;
        }
    }
    ans
}

fn main() {
    println!(
        "{:?}",
        count_mentions(
            2,
            vec![
                vec!["MESSAGE".into(), "10".into(), "id1 id0".into()],
                vec!["OFFLINE".into(), "11".into(), "0".into()],
                vec!["MESSAGE".into(), "71".into(), "HERE".into()],
            ]
        )
    );
}

#[cfg(test)]
mod tests {
    use super::count_mentions;

    #[test]
    fn example1() {
        assert_eq!(
            count_mentions(
                2,
                vec![
                    vec!["MESSAGE".into(), "10".into(), "id1 id0".into()],
                    vec!["OFFLINE".into(), "11".into(), "0".into()],
                    vec!["MESSAGE".into(), "71".into(), "HERE".into()],
                ]
            ),
            vec![2, 2]
        );
    }

    #[test]
    fn example2() {
        assert_eq!(
            count_mentions(
                2,
                vec![
                    vec!["MESSAGE".into(), "10".into(), "id1 id0".into()],
                    vec!["OFFLINE".into(), "11".into(), "0".into()],
                    vec!["MESSAGE".into(), "12".into(), "ALL".into()],
                ]
            ),
            vec![2, 2]
        );
    }

    #[test]
    fn example3() {
        assert_eq!(
            count_mentions(
                2,
                vec![
                    vec!["OFFLINE".into(), "10".into(), "0".into()],
                    vec!["MESSAGE".into(), "12".into(), "HERE".into()],
                ]
            ),
            vec![0, 1]
        );
    }
}
