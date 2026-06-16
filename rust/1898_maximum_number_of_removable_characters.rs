/// LeetCode #1898 - Maximum Number of Removable Characters
fn maximum_removals(s: String, p: String, removable: Vec<i32>) -> i32 {
    let s = s.as_bytes();
    let p = p.as_bytes();
    let check = |k: usize| -> bool {
        let mut rem = vec![false; s.len()];
        for &i in removable.iter().take(k) {
            rem[i as usize] = true;
        }
        let mut i = 0usize;
        let mut j = 0usize;
        while i < s.len() && j < p.len() {
            if !rem[i] && p[j] == s[i] {
                j += 1;
            }
            i += 1;
        }
        j == p.len()
    };

    let mut lo = 0usize;
    let mut hi = removable.len();
    while lo < hi {
        let mid = lo + (hi - lo + 1) / 2;
        if check(mid) {
            lo = mid;
        } else {
            hi = mid - 1;
        }
    }
    lo as i32
}

fn main() {
    println!(
        "{}",
        maximum_removals("abcacb".into(), "ab".into(), vec![3, 1, 0])
    );
}

#[cfg(test)]
mod tests {
    use super::maximum_removals;

    #[test]
    fn example_one() {
        assert_eq!(
            maximum_removals("abcacb".into(), "ab".into(), vec![3, 1, 0]),
            2
        );
    }

    #[test]
    fn example_two() {
        assert_eq!(
            maximum_removals("abcacb".into(), "ab".into(), vec![3, 1, 0, 4]),
            2
        );
    }
}
