/// LeetCode #3301 - Maximize the Total Height of Unique Towers
fn maximum_total_sum(mut maximum_height: Vec<i32>) -> i64 {
    maximum_height.sort_unstable_by(|a, b| b.cmp(a));
    let mut ans = 0i64;
    let mut mx = i32::MAX;
    for &h in &maximum_height {
        let x = h.min(mx - 1);
        if x <= 0 {
            return -1;
        }
        ans += x as i64;
        mx = x;
    }
    ans
}

fn main() {
    println!("{}", maximum_total_sum(vec![2, 3, 4, 3]));
}

#[cfg(test)]
mod tests {
    use super::maximum_total_sum;

    #[test]
    fn example1() {
        assert_eq!(maximum_total_sum(vec![2, 3, 4, 3]), 10);
    }

    #[test]
    fn example2() {
        assert_eq!(maximum_total_sum(vec![15, 10]), 25);
    }

    #[test]
    fn example3() {
        assert_eq!(maximum_total_sum(vec![2, 2, 1]), -1);
    }
}
