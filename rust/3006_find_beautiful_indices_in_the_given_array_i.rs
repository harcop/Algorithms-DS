/// LeetCode #3006 - Find Beautiful Indices in the Given Array I
fn beautiful_indices(s: String, a: String, b: String, k: i32) -> Vec<i32> {
    let s = s.as_bytes();
    let a = a.as_bytes();
    let b = b.as_bytes();
    let k = k as i64;

    let mut pos_a = Vec::new();
    let mut pos_b = Vec::new();

    for i in 0..s.len() {
        if i + a.len() <= s.len() && s[i..i + a.len()] == *a {
            pos_a.push(i as i64);
        }
        if i + b.len() <= s.len() && s[i..i + b.len()] == *b {
            pos_b.push(i as i64);
        }
    }

    let mut ans = Vec::new();
    let mut j = 0usize;
    for &i in &pos_a {
        while j < pos_b.len() && pos_b[j] < i - k {
            j += 1;
        }
        if j < pos_b.len() && (pos_b[j] - i).abs() <= k {
            ans.push(i as i32);
        }
    }
    ans
}

fn main() {
    println!(
        "{:?}",
        beautiful_indices(
            "isawsquirrelnearmysquirrelhouseohmy".into(),
            "my".into(),
            "squirrel".into(),
            15
        )
    );
    println!(
        "{:?}",
        beautiful_indices("abcd".into(), "a".into(), "a".into(), 4)
    );
}

#[cfg(test)]
mod tests {
    use super::beautiful_indices;

    #[test]
    fn example_one() {
        assert_eq!(
            beautiful_indices(
                "isawsquirrelnearmysquirrelhouseohmy".into(),
                "my".into(),
                "squirrel".into(),
                15
            ),
            vec![16, 33]
        );
    }

    #[test]
    fn example_two() {
        assert_eq!(
            beautiful_indices("abcd".into(), "a".into(), "a".into(), 4),
            vec![0]
        );
    }
}
