/// LeetCode #936 - Stamping The Sequence

fn moves_to_stamp(stamp: String, target: String) -> Vec<i32> {
    let stamp: Vec<char> = stamp.chars().collect();
    let mut target: Vec<char> = target.chars().collect();
    let m = stamp.len();
    let mut ans = Vec::new();
    loop {
        let mut found = false;
        for i in (0..=target.len().saturating_sub(m)).rev() {
            let ok = (0..m).all(|j| target[i + j] == '?' || target[i + j] == stamp[j])
                && (0..m).any(|j| target[i + j] != '?');
            if ok {
                ans.push(i as i32);
                for j in 0..m {
                    target[i + j] = '?';
                }
                found = true;
                break;
            }
        }
        if !found {
            break;
        }
    }
    if target.iter().all(|&c| c == '?') {
        ans.reverse();
        ans
    } else {
        vec![]
    }
}

fn main() {
    println!("{:?}", moves_to_stamp("abc".into(), "ababc".into()));
}

#[cfg(test)]
mod tests {
    use super::moves_to_stamp;

    #[test]
    fn example_one() {
        assert_eq!(moves_to_stamp("abc".into(), "ababc".into()), vec![0, 2]);
    }

    #[test]
    fn example_two() {
        let ans = moves_to_stamp("abca".into(), "aabcaca".into());
        assert_eq!(ans.len(), 3);
        assert!(ans == vec![3, 0, 1] || ans == vec![0, 3, 1]);
    }
}
