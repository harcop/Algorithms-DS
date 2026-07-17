/// LeetCode #2454 - Next Greater Element IV
fn second_greater_element(nums: Vec<i32>) -> Vec<i32> {
    let mut answer = vec![-1; nums.len()];
    let mut first = Vec::<usize>::new();
    let mut second = Vec::<usize>::new();

    for i in 0..nums.len() {
        while second.last().is_some_and(|&index| nums[index] < nums[i]) {
            answer[second.pop().unwrap()] = nums[i];
        }

        let mut moved = Vec::new();
        while first.last().is_some_and(|&index| nums[index] < nums[i]) {
            moved.push(first.pop().unwrap());
        }
        while let Some(index) = moved.pop() {
            second.push(index);
        }
        first.push(i);
    }

    answer
}

fn main() {
    println!("{:?}", second_greater_element(vec![2, 4, 0, 9, 6]));
}

#[cfg(test)]
mod tests {
    use super::second_greater_element;

    #[test]
    fn example_one() {
        assert_eq!(
            second_greater_element(vec![2, 4, 0, 9, 6]),
            vec![9, 6, 6, -1, -1]
        );
    }

    #[test]
    fn example_two() {
        assert_eq!(second_greater_element(vec![3, 3]), vec![-1, -1]);
    }
}
