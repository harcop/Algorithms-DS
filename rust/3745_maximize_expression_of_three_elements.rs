/// LeetCode #3745 - Maximize Expression of Three Elements
fn maximize_expression_of_three(nums: Vec<i32>) -> i32 {
    let mut a = i32::MIN;
    let mut b = i32::MIN;
    let mut c = i32::MAX;
    for x in nums {
        if x < c {
            c = x;
        }
        if x >= a {
            b = a;
            a = x;
        } else if x > b {
            b = x;
        }
    }
    a + b - c
}

fn main() {
    println!("{}", maximize_expression_of_three(vec![1, 4, 2, 5]));
}

#[cfg(test)]
mod tests {
    use super::maximize_expression_of_three;

    #[test]
    fn example1() {
        assert_eq!(maximize_expression_of_three(vec![1, 4, 2, 5]), 8);
    }

    #[test]
    fn example2() {
        assert_eq!(maximize_expression_of_three(vec![-2, 0, 5, -2, 4]), 11);
    }
}
