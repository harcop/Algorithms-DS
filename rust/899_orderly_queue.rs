/// LeetCode #899 - Orderly Queue
fn orderly_queue(s: String, k: i32) -> String {
    if k >= 2 {
        let mut c: Vec<char> = s.chars().collect();
        c.sort_unstable();
        return c.into_iter().collect();
    }
    let sb: Vec<char> = s.chars().collect();
    let n = sb.len();
    let mut best = sb.clone();
    for i in 1..n {
        let t: Vec<char> = sb[i..].iter().chain(sb[..i].iter()).copied().collect();
        if t < best {
            best = t;
        }
    }
    best.into_iter().collect()
}

fn main() {
    println!("{}", orderly_queue("cba".into(), 1));
}

#[cfg(test)]
mod tests {
    use super::orderly_queue;

    #[test]
    fn example_one() {
        assert_eq!(orderly_queue("cba".into(), 1), "acb");
    }
}
