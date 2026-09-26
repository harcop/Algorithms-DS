/// LeetCode #3952 - Maximum Total Value of Covered Indices
fn max_total(nums: Vec<i32>, s: String) -> i64 {
    let n = nums.len();
    let s = s.as_bytes();
    let mut nxt = [0i64, 0i64];
    for i in (0..n).rev() {
        let mut cur = [0i64, 0i64];
        for prev in 0..2 {
            if s[i] == b'1' {
                if prev == 1 {
                    cur[prev] = nums[i] as i64 + nxt[1];
                } else {
                    let stay = nums[i] as i64 + nxt[1];
                    let move_back = if i > 0 {
                        nums[i - 1] as i64 + nxt[0]
                    } else {
                        i64::MIN / 4
                    };
                    cur[prev] = stay.max(move_back);
                }
            } else {
                cur[prev] = nxt[0];
            }
        }
        nxt = cur;
    }
    nxt[0]
}

fn main() {
    println!("{}", max_total(vec![9, 2, 6, 1], "0101".into()));
}

#[cfg(test)]
mod tests {
    use super::max_total;

    #[test]
    fn example1() {
        assert_eq!(max_total(vec![9, 2, 6, 1], "0101".into()), 15);
    }

    #[test]
    fn example2() {
        assert_eq!(max_total(vec![5, 1, 4], "001".into()), 4);
    }

    #[test]
    fn example3() {
        assert_eq!(max_total(vec![9, 3, 5], "011".into()), 14);
    }
}
