/// LeetCode #3467 - Transform Array by Parity
fn transform_array(nums: Vec<i32>) -> Vec<i32> {
    let even = nums.iter().filter(|&&x| x % 2 == 0).count();
    let mut ans = vec![0; nums.len()];
    for x in ans.iter_mut().skip(even) {
        *x = 1;
    }
    ans
}

fn main() {
    println!("{:?}", transform_array(vec![4, 3, 2, 1]));
}

#[cfg(test)]
mod tests {
    use super::transform_array;

    #[test]
    fn example1() {
        assert_eq!(transform_array(vec![4, 3, 2, 1]), vec![0, 0, 1, 1]);
    }

    #[test]
    fn example2() {
        assert_eq!(transform_array(vec![1, 5, 1, 4, 2]), vec![0, 0, 1, 1, 1]);
    }
}
