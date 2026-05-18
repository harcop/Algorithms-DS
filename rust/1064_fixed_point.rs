/// LeetCode #1064 - Fixed Point
fn fixed_point(arr: Vec<i32>) -> i32 {
    for (i, &v) in arr.iter().enumerate() {
        if v == i as i32 {
            return i as i32;
        }
    }
    -1
}

fn main() {
    println!("{}", fixed_point(vec![-10, -5, 0, 3, 7]));
}

#[cfg(test)]
mod tests {
    use super::fixed_point;

    #[test]
    fn example_one() {
        assert_eq!(fixed_point(vec![-10, -5, 0, 3, 7]), 3);
    }

    #[test]
    fn example_two() {
        assert_eq!(fixed_point(vec![0, 2, 5, 8, 17]), 0);
    }
}
