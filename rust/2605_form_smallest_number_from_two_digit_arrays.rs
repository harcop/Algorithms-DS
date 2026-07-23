/// LeetCode #2605 - Form Smallest Number From Two Digit Arrays
fn min_number(nums1: Vec<i32>, nums2: Vec<i32>) -> i32 {
    let mut s1 = [false; 10];
    let mut s2 = [false; 10];
    for x in nums1 {
        s1[x as usize] = true;
    }
    for x in nums2 {
        s2[x as usize] = true;
    }
    let mut a = 0;
    let mut b = 0;
    for i in 1..10 {
        if s1[i] && s2[i] {
            return i as i32;
        }
        if a == 0 && s1[i] {
            a = i as i32;
        }
        if b == 0 && s2[i] {
            b = i as i32;
        }
    }
    (a * 10 + b).min(b * 10 + a)
}

fn main() {
    println!("{}", min_number(vec![4, 1, 3], vec![5, 7]));
}

#[cfg(test)]
mod tests {
    use super::min_number;

    #[test]
    fn example_one() {
        assert_eq!(min_number(vec![4, 1, 3], vec![5, 7]), 15);
    }

    #[test]
    fn example_two() {
        assert_eq!(min_number(vec![3, 5, 2, 6], vec![3, 1, 7]), 3);
    }
}
