/// LeetCode #3784 - Minimum Deletion Cost to Make All Characters Equal
fn min_cost(s: String, cost: Vec<i32>) -> i64 {
    let mut g = [0i64; 26];
    let mut tot = 0i64;
    for (c, v) in s.bytes().zip(cost) {
        tot += v as i64;
        g[(c - b'a') as usize] += v as i64;
    }
    tot - *g.iter().max().unwrap()
}

fn main() {
    println!("{}", min_cost("aabaac".into(), vec![1, 2, 3, 4, 1, 10]));
}

#[cfg(test)]
mod tests {
    use super::min_cost;

    #[test]
    fn example1() {
        assert_eq!(min_cost("aabaac".into(), vec![1, 2, 3, 4, 1, 10]), 11);
    }

    #[test]
    fn example2() {
        assert_eq!(min_cost("abc".into(), vec![10, 5, 8]), 13);
    }

    #[test]
    fn example3() {
        assert_eq!(min_cost("zzzzz".into(), vec![67, 67, 67, 67, 67]), 0);
    }
}
