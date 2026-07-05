/// LeetCode #2226 - Maximum Candies Allocated to K Children
fn maximum_candies(candies: Vec<i32>, k: i64) -> i32 {
    let mut l = 0i32;
    let mut r = *candies.iter().max().unwrap_or(&0);
    while l < r {
        let mid = (l + r + 1) / 2;
        let children: i64 = candies.iter().map(|&x| (x / mid) as i64).sum();
        if children >= k {
            l = mid;
        } else {
            r = mid - 1;
        }
    }
    l
}

fn main() {
    println!("{}", maximum_candies(vec![5, 8, 6], 3));
}

#[cfg(test)]
mod tests {
    use super::maximum_candies;

    #[test]
    fn example_one() {
        assert_eq!(maximum_candies(vec![5, 8, 6], 3), 5);
    }

    #[test]
    fn example_two() {
        assert_eq!(maximum_candies(vec![2, 5], 11), 0);
    }
}
