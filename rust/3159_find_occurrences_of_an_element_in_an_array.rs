/// LeetCode #3159 - Find Occurrences of an Element in an Array
fn occurrences_of_element(nums: Vec<i32>, queries: Vec<i32>, x: i32) -> Vec<i32> {
    let ids: Vec<i32> = nums
        .iter()
        .enumerate()
        .filter(|&(_, &v)| v == x)
        .map(|(i, _)| i as i32)
        .collect();
    queries
        .into_iter()
        .map(|q| {
            let i = (q - 1) as usize;
            if i < ids.len() {
                ids[i]
            } else {
                -1
            }
        })
        .collect()
}

fn main() {
    println!(
        "{:?}",
        occurrences_of_element(vec![1, 3, 1, 7], vec![1, 3, 2, 4], 1)
    );
}

#[cfg(test)]
mod tests {
    use super::occurrences_of_element;

    #[test]
    fn example1() {
        assert_eq!(
            occurrences_of_element(vec![1, 3, 1, 7], vec![1, 3, 2, 4], 1),
            vec![0, -1, 2, -1]
        );
    }

    #[test]
    fn example2() {
        assert_eq!(
            occurrences_of_element(vec![1, 2, 3], vec![10], 5),
            vec![-1]
        );
    }
}
