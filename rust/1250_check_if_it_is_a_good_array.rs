/// LeetCode #1250 - Check If It Is a Good Array
fn is_good_array(nums: Vec<i32>) -> bool {
    fn gcd(mut a: i64, mut b: i64) -> i64 {
        while b != 0 {
            let t = b;
            b = a % b;
            a = t;
        }
        a
    }
    let mut g = 0i64;
    for x in nums {
        g = gcd(g, x as i64);
        if g == 1 {
            return true;
        }
    }
    g == 1
}

fn main() {
    println!("{}", is_good_array(vec![12, 5, 7, 23]));
}

#[cfg(test)]
mod tests {
    use super::is_good_array;

    #[test]
    fn example_one() {
        assert!(is_good_array(vec![12, 5, 7, 23]));
    }

    #[test]
    fn example_two() {
        assert!(is_good_array(vec![29, 6, 10]));
    }

    #[test]
    fn example_three() {
        assert!(!is_good_array(vec![3, 6]));
    }
}
