/// LeetCode #396 - Rotate Function
fn max_rotate_function(nums: Vec<i32>) -> i32 {
    let n = nums.len() as i64;
    let sum: i64 = nums.iter().map(|&x| x as i64).sum();
    let mut f: i64 = nums.iter().enumerate().map(|(i,&x)| i as i64 * x as i64).sum();
    let mut best = f;
    for k in (1..nums.len()).rev() {
        f = f + sum - n * nums[k] as i64;
        best = best.max(f);
    }
    best as i32
}

fn main() {
    println!("{}", max_rotate_function(vec![4, 3, 2, 6]));
}

#[cfg(test)]
mod tests {
    use super::max_rotate_function;

    #[test]
    fn example_one() {
        assert_eq!(max_rotate_function(vec![4, 3, 2, 6]), 26);
    }
}
