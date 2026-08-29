/// LeetCode #3455 - Shortest Matching Substring
fn prefix_function(pat: &[u8]) -> Vec<usize> {
    let mut pi = vec![0; pat.len()];
    for i in 1..pat.len() {
        let mut j = pi[i - 1];
        while j > 0 && pat[j] != pat[i] {
            j = pi[j - 1];
        }
        if pat[j] == pat[i] {
            j += 1;
        }
        pi[i] = j;
    }
    pi
}

fn kmp_starts(text: &[u8], pat: &[u8]) -> Vec<usize> {
    if pat.is_empty() {
        return (0..=text.len()).collect();
    }
    let pi = prefix_function(pat);
    let mut res = Vec::new();
    let mut j = 0;
    for (i, &c) in text.iter().enumerate() {
        while j > 0 && pat[j] != c {
            j = pi[j - 1];
        }
        if pat[j] == c {
            j += 1;
        }
        if j == pat.len() {
            res.push(i + 1 - j);
            j = pi[j - 1];
        }
    }
    res
}

fn shortest_matching_substring(s: String, p: String) -> i32 {
    let parts: Vec<&str> = p.split('*').collect();
    let (a, b, c) = (parts[0].as_bytes(), parts[1].as_bytes(), parts[2].as_bytes());
    let s = s.as_bytes();
    let oa = kmp_starts(s, a);
    let ob = kmp_starts(s, b);
    let oc = kmp_starts(s, c);
    let mut ans = i32::MAX;
    for &ib in &ob {
        let lim_a = ib as i32 - a.len() as i32;
        let idx1 = oa.partition_point(|&x| (x as i32) <= lim_a) as i32 - 1;
        let lim_c = ib + b.len();
        let idx3 = oc.partition_point(|&x| x < lim_c);
        if idx1 >= 0 && idx3 < oc.len() {
            let i1 = oa[idx1 as usize];
            let j3 = oc[idx3] + c.len();
            ans = ans.min((j3 - i1) as i32);
        }
    }
    if ans == i32::MAX {
        -1
    } else {
        ans
    }
}

fn main() {
    println!(
        "{}",
        shortest_matching_substring("abaacbaecebce".into(), "ba*c*ce".into())
    );
}

#[cfg(test)]
mod tests {
    use super::shortest_matching_substring;

    #[test]
    fn example1() {
        assert_eq!(
            shortest_matching_substring("abaacbaecebce".into(), "ba*c*ce".into()),
            8
        );
    }

    #[test]
    fn example2() {
        assert_eq!(
            shortest_matching_substring("baccbaadbc".into(), "cc*baa*adb".into()),
            -1
        );
    }

    #[test]
    fn example3() {
        assert_eq!(shortest_matching_substring("a".into(), "**".into()), 0);
    }

    #[test]
    fn example4() {
        assert_eq!(
            shortest_matching_substring("madlogic".into(), "*adlogi*".into()),
            6
        );
    }
}
