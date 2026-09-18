/// LeetCode #3790 - Smallest All-Ones Multiple
fn min_all_one_multiple(k: i32) -> i32 {
    if k % 2 == 0 {
        return -1;
    }
    let mut x = 1 % k;
    let mut ans = 1;
    for _ in 0..k {
        x = (x * 10 + 1) % k;
        ans += 1;
        if x == 0 {
            return ans;
        }
    }
    -1
}

fn main() {
    println!("{}", min_all_one_multiple(3));
}

#[cfg(test)]
mod tests {
    use super::min_all_one_multiple;

    #[test]
    fn example1() {
        assert_eq!(min_all_one_multiple(3), 3);
    }

    #[test]
    fn example2() {
        assert_eq!(min_all_one_multiple(7), 6);
    }

    #[test]
    fn example3() {
        assert_eq!(min_all_one_multiple(2), -1);
    }
}
