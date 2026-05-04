/// LeetCode #299 - Bulls and Cows
fn get_hint(secret: String, guess: String) -> String {
    let s = secret.as_bytes();
    let g = guess.as_bytes();
    let mut bulls = 0;
    let mut cnt_s = [0i32; 10];
    let mut cnt_g = [0i32; 10];
    for i in 0..s.len() {
        if s[i] == g[i] {
            bulls += 1;
        } else {
            cnt_s[(s[i] - b'0') as usize] += 1;
            cnt_g[(g[i] - b'0') as usize] += 1;
        }
    }
    let cows: i32 = (0..10).map(|i| cnt_s[i].min(cnt_g[i])).sum();
    format!("{}A{}B", bulls, cows)
}

fn main() {
    println!("{}", get_hint("1807".into(), "7810".into()));
}

#[cfg(test)]
mod tests {
    use super::get_hint;

    #[test]
    fn example_one() {
        assert_eq!(get_hint("1807".into(), "7810".into()), "1A3B");
    }

    #[test]
    fn example_two() {
        assert_eq!(get_hint("1123".into(), "0111".into()), "1A1B");
    }
}
