/// LeetCode #3854 - Minimum Operations to Make Array Parity Alternating
fn make_parity_alternating(nums: Vec<i32>) -> Vec<i32> {
    if nums.len() == 1 {
        return vec![0, 0];
    }
    let mn = *nums.iter().min().unwrap();
    let mx = *nums.iter().max().unwrap();
    let f = |k: i32| -> Vec<i32> {
        let mut cnt = 0;
        let mut a = i32::MAX;
        let mut b = i32::MIN;
        for (i, &orig) in nums.iter().enumerate() {
            let mut x = orig;
            if ((x as i64 - i as i64) & 1) != k as i64 {
                cnt += 1;
                if x == mn {
                    x += 1;
                } else if x == mx {
                    x -= 1;
                }
            }
            a = a.min(x);
            b = b.max(x);
        }
        vec![cnt, 1.max(b - a)]
    };
    let r0 = f(0);
    let r1 = f(1);
    if r0 <= r1 {
        r0
    } else {
        r1
    }
}

fn main() {
    println!("{:?}", make_parity_alternating(vec![-2, -3, 1, 4]));
}

#[cfg(test)]
mod tests {
    use super::make_parity_alternating;

    #[test]
    fn example1() {
        assert_eq!(make_parity_alternating(vec![-2, -3, 1, 4]), vec![2, 6]);
    }

    #[test]
    fn example2() {
        assert_eq!(make_parity_alternating(vec![0, 2, -2]), vec![1, 3]);
    }

    #[test]
    fn example3() {
        assert_eq!(make_parity_alternating(vec![7]), vec![0, 0]);
    }
}
