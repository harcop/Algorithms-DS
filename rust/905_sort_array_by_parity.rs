/// LeetCode #905 - Sort Array By Parity
fn sort_array_by_parity(nums: Vec<i32>) -> Vec<i32> {
    let mut a = nums;
    let mut lo = 0usize;
    for i in 0..a.len() {
        if a[i] % 2 == 0 {
            a.swap(lo, i);
            lo += 1;
        }
    }
    a
}

fn main() {
    println!("{:?}", sort_array_by_parity(vec![3, 1, 2, 4]));
}

#[cfg(test)]
mod tests {
    use super::sort_array_by_parity;

    #[test]
    fn example_one() {
        let v = sort_array_by_parity(vec![3, 1, 2, 4]);
        assert_eq!(v.iter().filter(|x| *x % 2 == 0).count(), 2);
        assert_eq!(v.iter().filter(|x| *x % 2 == 1).count(), 2);
    }
}
