/// LeetCode #484 - Find Permutation
fn find_permutation(s: String) -> Vec<i32> {
    let n = s.len() + 1;
    let mut ans: Vec<i32> = (1..=n as i32).collect();
    let b = s.as_bytes();
    let mut i = 0;
    while i < b.len() {
        if b[i] == b'D' {
            let start = i;
            while i < b.len() && b[i] == b'D' {
                i += 1;
            }
            ans[start..=i].reverse();
        } else {
            i += 1;
        }
    }
    ans
}

fn main() {
    println!("{:?}", find_permutation("DI".into()));
}

#[cfg(test)]
mod tests {
    use super::find_permutation;

    #[test]
    fn example_one() {
        assert_eq!(find_permutation("I".into()), vec![1, 2]);
    }

    #[test]
    fn example_two() {
        assert_eq!(find_permutation("DI".into()), vec![2, 1, 3]);
    }
}
