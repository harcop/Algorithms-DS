/// LeetCode #1156 - Swap For Longest Repeated Character Substring
fn max_rep_opt1(text: String) -> i32 {
    let s: Vec<char> = text.chars().collect();
    let n = s.len();
    let mut total = [0usize; 26];
    for &c in &s {
        total[(c as u8 - b'a') as usize] += 1;
    }
    let mut groups: Vec<(char, usize)> = Vec::new();
    let mut i = 0;
    while i < n {
        let mut j = i;
        while j < n && s[j] == s[i] {
            j += 1;
        }
        groups.push((s[i], j - i));
        i = j;
    }
    let mut ans = 0usize;
    for &(c, len) in &groups {
        let t = total[(c as u8 - b'a') as usize];
        ans = ans.max(len.min(t).min(len + 1).min(t));
        ans = ans.max(len.min(t));
        if t > len {
            ans = ans.max(len + 1);
        } else {
            ans = ans.max(len);
        }
    }
    for w in groups.windows(3) {
        if w[0].0 == w[2].0 && w[1].1 == 1 {
            let c = w[0].0;
            let merged = w[0].1 + w[2].1;
            let t = total[(c as u8 - b'a') as usize];
            if t > merged {
                ans = ans.max(merged + 1);
            } else {
                ans = ans.max(merged);
            }
        }
    }
    ans as i32
}

fn main() {
    println!("{}", max_rep_opt1("ababa".into()));
}

#[cfg(test)]
mod tests {
    use super::max_rep_opt1;

    #[test]
    fn example_one() {
        assert_eq!(max_rep_opt1("ababa".into()), 3);
    }

    #[test]
    fn example_two() {
        assert_eq!(max_rep_opt1("aaabaaa".into()), 6);
    }

    #[test]
    fn example_three() {
        assert_eq!(max_rep_opt1("aaaaa".into()), 5);
    }
}
