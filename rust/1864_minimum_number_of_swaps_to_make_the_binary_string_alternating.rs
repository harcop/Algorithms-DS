/// LeetCode #1864 - Minimum Number of Swaps to Make the Binary String Alternating
fn min_swaps(s: String) -> i32 {
    let s = s.as_bytes();
    let n0 = s.iter().filter(|&&b| b == b'0').count();
    let n1 = s.len() - n0;
    if n0.abs_diff(n1) > 1 {
        return -1;
    }
    let calc = |c: usize| -> i32 {
        let mismatches = s
            .iter()
            .enumerate()
            .filter(|&(i, &x)| (c ^ (i & 1)) != (x - b'0') as usize)
            .count();
        (mismatches / 2) as i32
    };
    if n0 == n1 {
        calc(0).min(calc(1))
    } else {
        calc(if n0 > n1 { 0 } else { 1 })
    }
}

fn main() {
    println!("{}", min_swaps("111000".into()));
}

#[cfg(test)]
mod tests {
    use super::min_swaps;

    #[test]
    fn example_one() {
        assert_eq!(min_swaps("111000".into()), 1);
    }
}
