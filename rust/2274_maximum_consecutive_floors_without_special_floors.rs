/// LeetCode #2274 - Maximum Consecutive Floors Without Special Floors
fn max_consecutive(bottom: i32, top: i32, mut special: Vec<i32>) -> i32 {
    special.sort_unstable();
    let mut ans = 0;

    for w in special.windows(2) {
        ans = ans.max(w[1] - w[0] - 1);
    }

    ans.max(special[0] - bottom).max(top - special[special.len() - 1])
}

fn main() {
    println!("{}", max_consecutive(2, 9, vec![4, 6]));
}

#[cfg(test)]
mod tests {
    use super::max_consecutive;

    #[test]
    fn example_one() {
        assert_eq!(max_consecutive(2, 9, vec![4, 6]), 3);
    }

    #[test]
    fn example_two() {
        assert_eq!(max_consecutive(6, 8, vec![7, 6, 8]), 0);
    }
}
