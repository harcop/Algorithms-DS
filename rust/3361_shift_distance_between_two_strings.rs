/// LeetCode #3361 - Shift Distance Between Two Strings
fn shift_distance(s: String, t: String, next_cost: Vec<i32>, previous_cost: Vec<i32>) -> i64 {
    let m = 26usize;
    let mut s1 = vec![0i64; (m << 1) + 1];
    let mut s2 = vec![0i64; (m << 1) + 1];
    for i in 0..(m << 1) {
        s1[i + 1] = s1[i] + next_cost[i % m] as i64;
        s2[i + 1] = s2[i] + previous_cost[(i + 1) % m] as i64;
    }
    let mut ans = 0i64;
    for (a, b) in s.bytes().zip(t.bytes()) {
        let x = (a - b'a') as usize;
        let y = (b - b'a') as usize;
        let c1 = s1[if y < x { y + m } else { y }] - s1[x];
        let c2 = s2[if x < y { x + m } else { x }] - s2[y];
        ans += c1.min(c2);
    }
    ans
}

fn main() {
    println!(
        "{}",
        shift_distance(
            "abab".into(),
            "baba".into(),
            vec![100, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
            vec![1, 100, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
        )
    );
}

#[cfg(test)]
mod tests {
    use super::shift_distance;

    #[test]
    fn example1() {
        assert_eq!(
            shift_distance(
                "abab".into(),
                "baba".into(),
                vec![100, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                vec![1, 100, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
            ),
            2
        );
    }

    #[test]
    fn example2() {
        assert_eq!(
            shift_distance(
                "leet".into(),
                "code".into(),
                vec![1; 26],
                vec![1; 26],
            ),
            31
        );
    }
}
