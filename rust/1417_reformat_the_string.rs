/// LeetCode #1417 - Reformat The String
fn reformat(s: String) -> String {
    let mut letters = Vec::new();
    let mut digits = Vec::new();
    for c in s.chars() {
        if c.is_ascii_digit() {
            digits.push(c);
        } else {
            letters.push(c);
        }
    }
    let (mut a, mut b) = if letters.len() >= digits.len() {
        (letters, digits)
    } else {
        (digits, letters)
    };
    if a.len() > b.len() + 1 {
        return String::new();
    }
    let mut ans = String::new();
    let mut i = 0usize;
    let mut j = 0usize;
    while i < a.len() || j < b.len() {
        if ans.len() % 2 == 0 {
            if i < a.len() {
                ans.push(a[i]);
                i += 1;
            } else {
                ans.push(b[j]);
                j += 1;
            }
        } else if j < b.len() {
            ans.push(b[j]);
            j += 1;
        } else {
            ans.push(a[i]);
            i += 1;
        }
    }
    ans
}

fn main() {
    println!("{}", reformat("a1b2c3d4e5f6g7h8i9j0k".into()));
}

#[cfg(test)]
mod tests {
    use super::reformat;

    #[test]
    fn example_one() {
        assert_eq!(reformat("a1b2c3d4e5f6g7h8i9j0k".into()).len(), 21);
    }

    #[test]
    fn example_two() {
        assert_eq!(reformat("leetcode".into()), "");
    }
}

