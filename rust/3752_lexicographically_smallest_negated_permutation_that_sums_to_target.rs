/// LeetCode #3752 - Lexicographically Smallest Negated Permutation that Sums to Target
fn lex_smallest_negated_permutation(n: i32, target: i64) -> Vec<i32> {
    let n = n as i64;
    let s = n * (n + 1) / 2;
    if target > s || target < -s || (s - target) % 2 != 0 {
        return Vec::new();
    }
    let mut need = (s - target) / 2;
    let mut neg = vec![false; n as usize + 1];
    for i in (1..=n).rev() {
        if need >= i {
            neg[i as usize] = true;
            need -= i;
        }
    }
    let mut ans = Vec::with_capacity(n as usize);
    for i in (1..=n).rev() {
        if neg[i as usize] {
            ans.push(-(i as i32));
        }
    }
    for i in 1..=n {
        if !neg[i as usize] {
            ans.push(i as i32);
        }
    }
    ans
}

fn main() {
    println!("{:?}", lex_smallest_negated_permutation(3, 0));
}

#[cfg(test)]
mod tests {
    use super::lex_smallest_negated_permutation;

    #[test]
    fn example1() {
        assert_eq!(lex_smallest_negated_permutation(3, 0), vec![-3, 1, 2]);
    }

    #[test]
    fn example2() {
        assert_eq!(
            lex_smallest_negated_permutation(1, 10_000_000_000),
            Vec::<i32>::new()
        );
    }
}
