/// LeetCode #3170 - Lexicographically Minimum String After Removing Stars
fn clear_stars(s: String) -> String {
    let chars: Vec<char> = s.chars().collect();
    let n = chars.len();
    let mut g: Vec<Vec<usize>> = vec![vec![]; 26];
    let mut rem = vec![false; n];
    for (i, &ch) in chars.iter().enumerate() {
        if ch == '*' {
            rem[i] = true;
            for j in 0..26 {
                if let Some(idx) = g[j].pop() {
                    rem[idx] = true;
                    break;
                }
            }
        } else {
            g[(ch as u8 - b'a') as usize].push(i);
        }
    }
    chars
        .into_iter()
        .enumerate()
        .filter_map(|(i, ch)| if !rem[i] { Some(ch) } else { None })
        .collect()
}

fn main() {
    println!("{}", clear_stars("aaba*".into()));
}

#[cfg(test)]
mod tests {
    use super::clear_stars;

    #[test]
    fn example1() {
        assert_eq!(clear_stars("aaba*".into()), "aab");
    }

    #[test]
    fn example2() {
        assert_eq!(clear_stars("abc".into()), "abc");
    }
}
