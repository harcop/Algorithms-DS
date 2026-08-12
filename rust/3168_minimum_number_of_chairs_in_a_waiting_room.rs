/// LeetCode #3168 - Minimum Number of Chairs in a Waiting Room
fn minimum_chairs(s: String) -> i32 {
    let mut cnt = 0;
    let mut left = 0;
    for c in s.chars() {
        if c == 'E' {
            if left > 0 {
                left -= 1;
            } else {
                cnt += 1;
            }
        } else {
            left += 1;
        }
    }
    cnt
}

fn main() {
    println!("{}", minimum_chairs("EEEEEEE".into()));
}

#[cfg(test)]
mod tests {
    use super::minimum_chairs;

    #[test]
    fn example1() {
        assert_eq!(minimum_chairs("EEEEEEE".into()), 7);
    }

    #[test]
    fn example2() {
        assert_eq!(minimum_chairs("ELELEEL".into()), 2);
    }

    #[test]
    fn example3() {
        assert_eq!(minimum_chairs("ELEELEELLL".into()), 3);
    }
}
