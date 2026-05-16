/// LeetCode #830 - Positions of Large Groups
fn large_group_positions(s: String) -> Vec<Vec<i32>> {
    let b = s.as_bytes();
    let mut res = Vec::new();
    let mut i = 0;
    while i < b.len() {
        let mut j = i;
        while j + 1 < b.len() && b[j + 1] == b[i] {
            j += 1;
        }
        if j - i + 1 >= 3 {
            res.push(vec![i as i32, j as i32]);
        }
        i = j + 1;
    }
    res
}

fn main() {
    println!("{:?}", large_group_positions("abbxxxyyzz".into()));
}

#[cfg(test)]
mod tests {
    use super::large_group_positions;

    #[test]
    fn example_one() {
        assert_eq!(large_group_positions("abbxxxyyzz".into()), vec![vec![3, 5]]);
    }
}
