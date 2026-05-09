/// LeetCode #565 - Array Nesting
fn array_nesting(nums: Vec<i32>) -> i32 {
    let n = nums.len();
    let mut seen = vec![false; n];
    let mut best = 0i32;
    for i in 0..n {
        if seen[i] {
            continue;
        }
        let mut c = 0i32;
        let mut j = i;
        while !seen[j] {
            seen[j] = true;
            j = nums[j] as usize;
            c += 1;
        }
        best = best.max(c);
    }
    best
}

fn main() {
    println!("{}", array_nesting(vec![5, 4, 0, 3, 1, 6, 2]));
}

#[cfg(test)]
mod tests {
    use super::array_nesting;

    #[test]
    fn example_one() {
        assert_eq!(array_nesting(vec![5, 4, 0, 3, 1, 6, 2]), 4);
    }
}
