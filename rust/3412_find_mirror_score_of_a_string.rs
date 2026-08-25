/// LeetCode #3412 - Find Mirror Score of a String
fn calculate_score(s: String) -> i64 {
    let mut d: Vec<Vec<usize>> = vec![Vec::new(); 26];
    let mut ans = 0i64;
    for (i, x) in s.bytes().enumerate() {
        let y = b'a' + b'z' - x;
        let yi = (y - b'a') as usize;
        let xi = (x - b'a') as usize;
        if let Some(j) = d[yi].pop() {
            ans += (i - j) as i64;
        } else {
            d[xi].push(i);
        }
    }
    ans
}

fn main() {
    println!("{}", calculate_score("aczzx".into()));
}

#[cfg(test)]
mod tests {
    use super::calculate_score;

    #[test]
    fn example1() {
        assert_eq!(calculate_score("aczzx".into()), 5);
    }

    #[test]
    fn example2() {
        assert_eq!(calculate_score("abcdef".into()), 0);
    }
}
