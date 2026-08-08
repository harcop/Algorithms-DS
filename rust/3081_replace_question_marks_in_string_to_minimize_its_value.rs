/// LeetCode #3081 - Replace Question Marks in String to Minimize Its Value
fn minimize_string_value(s: String) -> String {
    let mut freq = [0i32; 26];
    let mut q = 0usize;
    for ch in s.chars() {
        if ch == '?' {
            q += 1;
        } else {
            freq[(ch as u8 - b'a') as usize] += 1;
        }
    }

    // Greedily assign '?' to letters that currently have smallest frequency
    // to minimize future contribution; collect letters then sort for lex-smallest placement.
    let mut assigned = Vec::with_capacity(q);
    let mut cur = freq;
    for _ in 0..q {
        let mut best = 0usize;
        for c in 1..26 {
            if cur[c] < cur[best] {
                best = c;
            }
        }
        assigned.push((b'a' + best as u8) as char);
        cur[best] += 1;
    }
    assigned.sort_unstable();

    let mut it = assigned.into_iter();
    let mut out = String::with_capacity(s.len());
    for ch in s.chars() {
        if ch == '?' {
            out.push(it.next().unwrap());
        } else {
            out.push(ch);
        }
    }
    out
}

fn main() {
    println!("{}", minimize_string_value("???".into()));
}

#[cfg(test)]
mod tests {
    use super::minimize_string_value;

    #[test]
    fn example1() {
        assert_eq!(minimize_string_value("???".into()), "abc");
    }

    #[test]
    fn example2() {
        assert_eq!(minimize_string_value("a?a?".into()), "abac");
    }
}
