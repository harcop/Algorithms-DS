/// LeetCode #2342 - Max Sum of a Pair With Equal Sum of Digits
fn maximum_sum(nums: Vec<i32>) -> i32 {
    let mut d = vec![0i32; 100];
    let mut ans = -1;

    for &v in &nums {
        let mut x = 0usize;
        let mut y = v;
        while y > 0 {
            x += (y % 10) as usize;
            y /= 10;
        }
        if d[x] > 0 {
            ans = ans.max(d[x] + v);
        }
        d[x] = d[x].max(v);
    }

    ans
}

fn main() {
    println!("{}", maximum_sum(vec![18, 43, 36, 13, 7]));
}

#[cfg(test)]
mod tests {
    use super::maximum_sum;

    #[test]
    fn example_one() {
        assert_eq!(maximum_sum(vec![18, 43, 36, 13, 7]), 54);
    }

    #[test]
    fn example_two() {
        assert_eq!(maximum_sum(vec![10, 12, 19, 14]), -1);
    }
}
