/// LeetCode #1390 - Four Divisors
fn sum_four_divisors(nums: Vec<i32>) -> i32 {
    fn div_sum(x: i32) -> Option<i32> {
        let mut cnt = 0;
        let mut s = 0;
        let mut i = 1i32;
        while i * i <= x {
            if x % i == 0 {
                cnt += 1;
                s += i;
                if i * i != x {
                    cnt += 1;
                    s += x / i;
                }
            }
            i += 1;
        }
        if cnt == 4 { Some(s) } else { None }
    }
    nums.into_iter().filter_map(div_sum).sum()
}

fn main() {
    println!("{}", sum_four_divisors(vec![21, 4, 7]));
}

#[cfg(test)]
mod tests {
    use super::sum_four_divisors;

    #[test]
    fn example_one() {
        assert_eq!(sum_four_divisors(vec![21, 4, 7]), 32);
    }

    #[test]
    fn example_two() {
        assert_eq!(sum_four_divisors(vec![21, 21]), 64);
    }
}

