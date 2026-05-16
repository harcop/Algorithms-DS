/// LeetCode #848 - Shifting Letters
fn shifting_letters(s: String, shifts: Vec<i32>) -> String {
    let mut sum = 0i32;
    let mut res = Vec::with_capacity(s.len());
    for (c, sh) in s.bytes().rev().zip(shifts.into_iter().rev()) {
        sum = (sum + sh).rem_euclid(26);
        let nc = ((c - b'a') as i32 + sum).rem_euclid(26) as u8 + b'a';
        res.push(nc);
    }
    res.reverse();
    String::from_utf8(res).unwrap()
}

fn main() {
    println!("{}", shifting_letters("abc".into(), vec![3, 5, 9]));
}

#[cfg(test)]
mod tests {
    use super::shifting_letters;

    #[test]
    fn example_one() {
        assert_eq!(shifting_letters("abc".into(), vec![3, 5, 9]), "rpl");
    }
}
