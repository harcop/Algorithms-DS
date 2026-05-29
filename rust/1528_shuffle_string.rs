/// LeetCode #1528 - Shuffle String
fn restore_string(s: String, indices: Vec<i32>) -> String {
    let mut ans = vec![' '; s.len()];
    for (ch, &idx) in s.chars().zip(indices.iter()) {
        ans[idx as usize] = ch;
    }
    ans.into_iter().collect()
}

fn main() {
    println!("{}", restore_string("codeleet".into(), vec![4, 5, 6, 7, 0, 2, 1, 3]));
}

#[cfg(test)]
mod tests {
    use super::restore_string;

    #[test]
    fn example_one() {
        assert_eq!(restore_string("codeleet".into(), vec![4, 5, 6, 7, 0, 2, 1, 3]), "leetcode");
    }
}
