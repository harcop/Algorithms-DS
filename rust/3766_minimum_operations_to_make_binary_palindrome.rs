/// LeetCode #3766 - Minimum Operations to Make Binary Palindrome
fn min_operations(nums: Vec<i32>) -> Vec<i32> {
    let mut pals = Vec::new();
    for i in 0..(1 << 14) {
        let s = format!("{i:b}");
        if s.as_bytes().iter().eq(s.as_bytes().iter().rev()) {
            pals.push(i);
        }
    }
    nums.into_iter()
        .map(|x| {
            let i = pals.partition_point(|&p| p < x);
            let mut t = i32::MAX;
            if i < pals.len() {
                t = t.min(pals[i] - x);
            }
            if i >= 1 {
                t = t.min(x - pals[i - 1]);
            }
            t
        })
        .collect()
}

fn main() {
    println!("{:?}", min_operations(vec![1, 2, 4]));
}

#[cfg(test)]
mod tests {
    use super::min_operations;

    #[test]
    fn example1() {
        assert_eq!(min_operations(vec![1, 2, 4]), vec![0, 1, 1]);
    }

    #[test]
    fn example2() {
        assert_eq!(min_operations(vec![6, 7, 12]), vec![1, 0, 3]);
    }
}
