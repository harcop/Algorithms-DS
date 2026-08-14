/// LeetCode #3208 - Alternating Groups II
fn number_of_alternating_groups(colors: Vec<i32>, k: i32) -> i32 {
    let k = k as usize;
    let n = colors.len();
    let mut ans = 0;
    let mut cnt = 0;
    for i in 0..(n * 2) {
        if i > 0 && colors[i % n] == colors[(i - 1) % n] {
            cnt = 1;
        } else {
            cnt += 1;
        }
        if i >= n && cnt >= k {
            ans += 1;
        }
    }
    ans
}

fn main() {
    println!(
        "{}",
        number_of_alternating_groups(vec![0, 1, 0, 1, 0], 3)
    );
}

#[cfg(test)]
mod tests {
    use super::number_of_alternating_groups;

    #[test]
    fn example1() {
        assert_eq!(
            number_of_alternating_groups(vec![0, 1, 0, 1, 0], 3),
            3
        );
    }

    #[test]
    fn example2() {
        assert_eq!(
            number_of_alternating_groups(vec![0, 1, 0, 0, 1, 0, 1], 6),
            2
        );
    }

    #[test]
    fn example3() {
        assert_eq!(number_of_alternating_groups(vec![1, 1, 0, 1], 4), 0);
    }
}
