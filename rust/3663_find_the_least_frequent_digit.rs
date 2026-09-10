/// LeetCode #3663 - Find The Least Frequent Digit
fn get_least_frequent_digit(mut n: i32) -> i32 {
    let mut cnt = [0i32; 10];
    while n > 0 {
        cnt[(n % 10) as usize] += 1;
        n /= 10;
    }
    let mut ans = 0;
    let mut f = i32::MAX;
    for x in 0..10 {
        if cnt[x] > 0 && cnt[x] < f {
            f = cnt[x];
            ans = x as i32;
        }
    }
    ans
}

fn main() {
    println!("{}", get_least_frequent_digit(1553322));
}

#[cfg(test)]
mod tests {
    use super::get_least_frequent_digit;

    #[test]
    fn example1() {
        assert_eq!(get_least_frequent_digit(1553322), 1);
    }

    #[test]
    fn example2() {
        assert_eq!(get_least_frequent_digit(723344511), 2);
    }
}
