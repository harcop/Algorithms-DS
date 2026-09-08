/// LeetCode #3638 - Maximum Balanced Shipments
fn max_balanced_shipments(weight: Vec<i32>) -> i32 {
    let mut ans = 0;
    let mut mx = 0;
    for x in weight {
        mx = mx.max(x);
        if x < mx {
            ans += 1;
            mx = 0;
        }
    }
    ans
}

fn main() {
    println!("{}", max_balanced_shipments(vec![2, 5, 1, 4, 3]));
}

#[cfg(test)]
mod tests {
    use super::max_balanced_shipments;

    #[test]
    fn example1() {
        assert_eq!(max_balanced_shipments(vec![2, 5, 1, 4, 3]), 2);
    }

    #[test]
    fn example2() {
        assert_eq!(max_balanced_shipments(vec![4, 4]), 0);
    }
}
