/// LeetCode #283 - Move Zeroes
fn move_zeroes(nums: &mut Vec<i32>) {
    let mut w = 0usize;
    for r in 0..nums.len() {
        if nums[r] != 0 {
            nums.swap(w, r);
            w += 1;
        }
    }
}

fn main() {
    let mut v = vec![0, 1, 0, 3, 12];
    move_zeroes(&mut v);
    println!("{:?}", v);
}

#[cfg(test)]
mod tests {
    use super::move_zeroes;

    #[test]
    fn example_one() {
        let mut v = vec![0, 1, 0, 3, 12];
        move_zeroes(&mut v);
        assert_eq!(v, vec![1, 3, 12, 0, 0]);
    }
}
