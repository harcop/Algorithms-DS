/// LeetCode #3206 - Alternating Groups I
fn number_of_alternating_groups(colors: Vec<i32>) -> i32 {
    let k = 3;
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
    println!("{}", number_of_alternating_groups(vec![0, 1, 0, 0, 1]));
}

#[cfg(test)]
mod tests {
    use super::number_of_alternating_groups;

    #[test]
    fn example1() {
        assert_eq!(number_of_alternating_groups(vec![1, 1, 1]), 0);
    }

    #[test]
    fn example2() {
        assert_eq!(number_of_alternating_groups(vec![0, 1, 0, 0, 1]), 3);
    }
}
