/// LeetCode #3132 - Find the Integer Added to Array II
fn minimum_added_integer(mut nums1: Vec<i32>, mut nums2: Vec<i32>) -> i32 {
    nums1.sort_unstable();
    nums2.sort_unstable();
    let f = |x: i32| -> bool {
        let mut i = 0;
        let mut j = 0;
        let mut cnt = 0;
        while i < nums1.len() && j < nums2.len() {
            if nums2[j] - nums1[i] != x {
                cnt += 1;
            } else {
                j += 1;
            }
            i += 1;
        }
        cnt <= 2
    };
    let mut ans = 1 << 30;
    for i in 0..3 {
        let x = nums2[0] - nums1[i];
        if f(x) {
            ans = ans.min(x);
        }
    }
    ans
}

fn main() {
    println!(
        "{}",
        minimum_added_integer(vec![4, 20, 16, 12, 8], vec![14, 18, 10])
    );
}

#[cfg(test)]
mod tests {
    use super::minimum_added_integer;

    #[test]
    fn example1() {
        assert_eq!(
            minimum_added_integer(vec![4, 20, 16, 12, 8], vec![14, 18, 10]),
            -2
        );
    }

    #[test]
    fn example2() {
        assert_eq!(minimum_added_integer(vec![3, 5, 5, 3], vec![7, 7]), 2);
    }
}
