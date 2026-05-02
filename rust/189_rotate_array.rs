/// LeetCode #189 - Rotate Array
fn rotate(nums: &mut Vec<i32>, k: i32) {
    let n = nums.len();
    if n == 0 {
        return;
    }
    let k = (k as usize) % n;
    if k == 0 {
        return;
    }
    nums.reverse();
    nums[..k].reverse();
    nums[k..].reverse();
}

fn main() {
    let mut v = vec![1, 2, 3, 4, 5, 6, 7];
    rotate(&mut v, 3);
    println!("{:?}", v);
}

#[cfg(test)]
mod tests {
    use super::rotate;

    #[test]
    fn example_one() {
        let mut v = vec![1, 2, 3, 4, 5, 6, 7];
        rotate(&mut v, 3);
        assert_eq!(v, vec![5, 6, 7, 1, 2, 3, 4]);
    }

    #[test]
    fn example_two() {
        let mut v = vec![-1, -100, 3, 99];
        rotate(&mut v, 2);
        assert_eq!(v, vec![3, 99, -1, -100]);
    }
}
