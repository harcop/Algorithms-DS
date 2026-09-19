/// LeetCode #3819 - Rotate Non Negative Elements
fn rotate_elements(mut nums: Vec<i32>, k: i32) -> Vec<i32> {
    let t: Vec<i32> = nums.iter().copied().filter(|&x| x >= 0).collect();
    let m = t.len();
    if m == 0 {
        return nums;
    }
    let mut d = vec![0; m];
    let k = k as i64;
    let m64 = m as i64;
    for (i, x) in t.into_iter().enumerate() {
        let pos = ((i as i64 - k).rem_euclid(m64)) as usize;
        d[pos] = x;
    }
    let mut j = 0;
    for x in nums.iter_mut() {
        if *x >= 0 {
            *x = d[j];
            j += 1;
        }
    }
    nums
}

fn main() {
    println!("{:?}", rotate_elements(vec![1, -2, 3, -4], 3));
}

#[cfg(test)]
mod tests {
    use super::rotate_elements;

    #[test]
    fn example1() {
        assert_eq!(rotate_elements(vec![1, -2, 3, -4], 3), vec![3, -2, 1, -4]);
    }

    #[test]
    fn example2() {
        assert_eq!(rotate_elements(vec![-3, -2, 7], 1), vec![-3, -2, 7]);
    }

    #[test]
    fn example3() {
        assert_eq!(rotate_elements(vec![5, 4, -9, 6], 2), vec![6, 5, -9, 4]);
    }
}
