/// LeetCode #2802 - Find The K-th Lucky Number
fn kth_lucky_number(mut k: i32) -> String {
    let mut n = 1;
    while k > 1 << n {
        k -= 1 << n;
        n += 1;
    }
    let mut ans = String::new();
    while n > 0 {
        n -= 1;
        if k <= 1 << n {
            ans.push('4');
        } else {
            ans.push('7');
            k -= 1 << n;
        }
    }
    ans
}

fn main() {
    println!("{}", kth_lucky_number(4));
}

#[cfg(test)]
mod tests {
    use super::kth_lucky_number;

    #[test]
    fn example_one() {
        assert_eq!(kth_lucky_number(4), "47");
    }

    #[test]
    fn example_two() {
        assert_eq!(kth_lucky_number(10), "477");
    }

    #[test]
    fn example_three() {
        assert_eq!(kth_lucky_number(1000), "777747447");
    }
}
