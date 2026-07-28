/// LeetCode #2748 - Number of Beautiful Pairs
fn gcd(mut a: i32, mut b: i32) -> i32 {
    while b != 0 {
        let t = a % b;
        a = b;
        b = t;
    }
    a
}

fn count_beautiful_pairs(nums: Vec<i32>) -> i32 {
    let mut cnt = [0i32; 10];
    let mut ans = 0;
    for x in nums {
        let last = x % 10;
        for y in 0..10 {
            if cnt[y] > 0 && gcd(last, y as i32) == 1 {
                ans += cnt[y];
            }
        }
        let mut first = x;
        while first > 9 {
            first /= 10;
        }
        cnt[first as usize] += 1;
    }
    ans
}

fn main() {
    println!("{}", count_beautiful_pairs(vec![2, 5, 1, 4]));
}

#[cfg(test)]
mod tests {
    use super::count_beautiful_pairs;

    #[test]
    fn example_one() {
        assert_eq!(count_beautiful_pairs(vec![2, 5, 1, 4]), 5);
    }

    #[test]
    fn example_two() {
        assert_eq!(count_beautiful_pairs(vec![11, 21, 12]), 2);
    }
}
