/// LeetCode #3769 - Sort Integers by Binary Reflection
fn sort_by_reflection(mut nums: Vec<i32>) -> Vec<i32> {
    fn reflect(mut x: i32) -> i32 {
        let mut y = 0;
        while x != 0 {
            y = (y << 1) | (x & 1);
            x >>= 1;
        }
        y
    }
    nums.sort_by_key(|&x| (reflect(x), x));
    nums
}

fn main() {
    println!("{:?}", sort_by_reflection(vec![4, 5, 4]));
}

#[cfg(test)]
mod tests {
    use super::sort_by_reflection;

    #[test]
    fn example1() {
        assert_eq!(sort_by_reflection(vec![4, 5, 4]), vec![4, 4, 5]);
    }

    #[test]
    fn example2() {
        assert_eq!(sort_by_reflection(vec![3, 6, 5, 8]), vec![8, 3, 6, 5]);
    }
}
