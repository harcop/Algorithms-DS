/// LeetCode #2982 - Find Longest Special Substring That Occurs Thrice II
fn maximum_length(s: String) -> i32 {
    let bytes = s.as_bytes();
    let n = bytes.len();
    let check = |x: usize| -> bool {
        let mut cnt = [0i32; 26];
        let mut i = 0;
        while i < n {
            let mut j = i + 1;
            while j < n && bytes[j] == bytes[i] {
                j += 1;
            }
            let len = j - i;
            if len >= x {
                cnt[(bytes[i] - b'a') as usize] += (len - x + 1) as i32;
            }
            i = j;
        }
        *cnt.iter().max().unwrap() >= 3
    };
    let mut l = 0usize;
    let mut r = n;
    while l < r {
        let mid = (l + r + 1) / 2;
        if check(mid) {
            l = mid;
        } else {
            r = mid - 1;
        }
    }
    if l == 0 {
        -1
    } else {
        l as i32
    }
}

fn main() {
    println!("{}", maximum_length("aaaa".into()));
}

#[cfg(test)]
mod tests {
    use super::maximum_length;

    #[test]
    fn example_one() {
        assert_eq!(maximum_length("aaaa".into()), 2);
    }

    #[test]
    fn example_two() {
        assert_eq!(maximum_length("abcdef".into()), -1);
    }

    #[test]
    fn example_three() {
        assert_eq!(maximum_length("abcaba".into()), 1);
    }
}
