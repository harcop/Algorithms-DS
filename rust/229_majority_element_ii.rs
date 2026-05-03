/// LeetCode #229 - Majority Element II
fn majority_element(nums: Vec<i32>) -> Vec<i32> {
    let mut a = 0i32;
    let mut b = 1i32;
    let mut ca = 0i32;
    let mut cb = 0i32;
    for &x in &nums {
        if x == a {
            ca += 1;
        } else if x == b {
            cb += 1;
        } else if ca == 0 {
            a = x;
            ca = 1;
        } else if cb == 0 {
            b = x;
            cb = 1;
        } else {
            ca -= 1;
            cb -= 1;
        }
    }
    ca = nums.iter().filter(|&&v| v == a).count() as i32;
    cb = nums.iter().filter(|&&v| v == b).count() as i32;
    let n = nums.len() as i32;
    let mut out = vec![];
    if ca > n / 3 {
        out.push(a);
    }
    if cb > n / 3 && a != b {
        out.push(b);
    }
    out.sort();
    out
}

fn main() {
    println!("{:?}", majority_element(vec![3, 2, 3]));
}

#[cfg(test)]
mod tests {
    use super::majority_element;

    #[test]
    fn example_one() {
        assert_eq!(majority_element(vec![3, 2, 3]), vec![3]);
    }

    #[test]
    fn example_two() {
        let mut v = majority_element(vec![1]);
        v.sort();
        assert_eq!(v, vec![1]);
    }
}
