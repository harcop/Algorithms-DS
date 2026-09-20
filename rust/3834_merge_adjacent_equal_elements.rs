/// LeetCode #3834 - Merge Adjacent Equal Elements
fn merge_adjacent(nums: Vec<i32>) -> Vec<i64> {
    let mut stk: Vec<i64> = Vec::new();
    for x in nums {
        stk.push(x as i64);
        while stk.len() > 1 && stk[stk.len() - 1] == stk[stk.len() - 2] {
            let a = stk.pop().unwrap();
            let b = stk.pop().unwrap();
            stk.push(a + b);
        }
    }
    stk
}

fn main() {
    println!("{:?}", merge_adjacent(vec![3, 1, 1, 2]));
}

#[cfg(test)]
mod tests {
    use super::merge_adjacent;

    #[test]
    fn example1() {
        assert_eq!(merge_adjacent(vec![3, 1, 1, 2]), vec![3, 4]);
    }

    #[test]
    fn example2() {
        assert_eq!(merge_adjacent(vec![2, 2, 4]), vec![8]);
    }

    #[test]
    fn example3() {
        assert_eq!(merge_adjacent(vec![3, 7, 5]), vec![3, 7, 5]);
    }
}
