/// LeetCode #3024 - Type of Triangle
fn triangle_type(nums: Vec<i32>) -> String {
    let mut sides = nums;
    sides.sort_unstable();
    let (a, b, c) = (sides[0], sides[1], sides[2]);
    if a + b <= c {
        return "none".into();
    }
    if a == c {
        return "equilateral".into();
    }
    if a == b || b == c {
        return "isosceles".into();
    }
    "scalene".into()
}

fn main() {
    println!("{}", triangle_type(vec![3, 3, 3]));
}

#[cfg(test)]
mod tests {
    use super::triangle_type;

    #[test]
    fn example1() {
        assert_eq!(triangle_type(vec![3, 3, 3]), "equilateral");
    }

    #[test]
    fn example2() {
        assert_eq!(triangle_type(vec![3, 4, 5]), "scalene");
    }

    #[test]
    fn example3() {
        assert_eq!(triangle_type(vec![5, 3, 5]), "isosceles");
    }
}
