/// LeetCode #2495 - Number of Subarrays Having Even Product
fn even_product(nums: Vec<i32>) -> i64 {
    let mut ans = 0i64;
    let mut last = -1i64;
    for (i, v) in nums.into_iter().enumerate() {
        if v % 2 == 0 {
            last = i as i64;
        }
        ans += last + 1;
    }
    ans
}

fn main() {
    println!("{}", even_product(vec![9, 6, 7, 13]));
}

#[cfg(test)]
mod tests {
    use super::even_product;

    #[test]
    fn example_one() {
        assert_eq!(even_product(vec![9, 6, 7, 13]), 6);
    }

    #[test]
    fn example_two() {
        assert_eq!(even_product(vec![7, 3, 5]), 0);
    }
}
