/// LeetCode #335 - Self Crossing
fn is_self_crossing(distance: Vec<i32>) -> bool {
    let n = distance.len();
    if n <= 3 {
        return false;
    }
    for i in 3..n {
        let i = i as isize;
        let d = |k: isize| distance[k as usize];
        if d(i) >= d(i - 2) && d(i - 1) <= d(i - 3) {
            return true;
        }
        if i >= 4 && d(i - 3) == d(i - 1) && d(i) + d(i - 4) >= d(i - 2) {
            return true;
        }
        if i >= 5
            && d(i - 5) <= d(i - 3)
            && d(i - 3) <= d(i - 1)
            && d(i - 1) <= d(i - 3) + d(i - 5)
            && d(i) >= d(i - 2) - d(i - 4)
            && d(i - 2) > d(i - 4)
        {
            return true;
        }
    }
    false
}

fn main() {
    println!("{}", is_self_crossing(vec![2, 1, 1, 2]));
}

#[cfg(test)]
mod tests {
    use super::is_self_crossing;

    #[test]
    fn examples() {
        assert!(is_self_crossing(vec![2, 1, 1, 2]));
        assert!(!is_self_crossing(vec![1, 2, 3, 4]));
        assert!(is_self_crossing(vec![1, 1, 1, 1]));
    }
}
