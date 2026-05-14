/// LeetCode #767 - Reorganize String
fn reorganize_string(s: String) -> String {
    let mut cnt = vec![0i32; 26];
    for b in s.bytes() {
        cnt[(b - b'a') as usize] += 1;
    }
    let mut res: Vec<u8> = Vec::with_capacity(s.len());
    for _ in 0..s.len() {
        let mut pick: Option<usize> = None;
        for j in 0..26 {
            if cnt[j] <= 0 {
                continue;
            }
            let ch = b'a' + j as u8;
            let ok = res.last().map(|&c| c != ch).unwrap_or(true);
            if !ok {
                continue;
            }
            match pick {
                None => pick = Some(j),
                Some(p) if cnt[j] > cnt[p] => pick = Some(j),
                _ => {}
            }
        }
        let j = match pick {
            Some(x) => x,
            None => return String::new(),
        };
        res.push(b'a' + j as u8);
        cnt[j] -= 1;
    }
    String::from_utf8(res).unwrap()
}

fn main() {
    println!("{}", reorganize_string("aab".into()));
}

#[cfg(test)]
mod tests {
    use super::reorganize_string;

    #[test]
    fn example_one() {
        let r = reorganize_string("aab".into());
        assert_eq!(r.len(), 3);
        assert!(!r.as_bytes().windows(2).any(|w| w[0] == w[1]));
    }
}
