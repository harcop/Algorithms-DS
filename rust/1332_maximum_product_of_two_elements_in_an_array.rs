/// LeetCode #1332 - Maximum Product of Two Elements in an Array
fn max_product(nums: Vec<i32>) -> i32 {
    let mut a = 0i32;
    let mut b = 0i32;
    for &x in &nums {
        if x > a {
            b = a;
            a = x;
        } else if x > b {
            b = x;
        }
    }
    (a - 1) * (b - 1)
}

fn main() {
    println!("{}", max_product(vec![3, 4, 5, 2]));
}

#[cfg(test)]
mod tests {
    use super::max_product;

    #[test]
    fn example_one() {
        assert_eq!(max_product(vec![3, 4, 5, 2]), 12);
    }

    #[test]
    fn example_two() {
        assert_eq!(max_product(vec![1, 5, 4, 5]), 16);
    }
}
