/// LeetCode #1385 - Find The Distance Value Between Two Arrays
fn find_the_distance_value(arr1: Vec<i32>, arr2: Vec<i32>, d: i32) -> i32 {
    let mut b = arr2;
    b.sort_unstable();
    arr1
        .into_iter()
        .filter(|&x| {
            let pos = b.partition_point(|&v| v < x - d);
            pos == b.len() || b[pos] > x + d
        })
        .count() as i32
}

fn main() {
    println!("{}", find_the_distance_value(vec![4, 5, 8], vec![10, 9, 1, 8], 2));
}

#[cfg(test)]
mod tests {
    use super::find_the_distance_value;

    #[test]
    fn example_one() {
        assert_eq!(find_the_distance_value(vec![4, 5, 8], vec![10, 9, 1, 8], 2), 2);
    }

    #[test]
    fn example_two() {
        assert_eq!(find_the_distance_value(vec![1, 4, 2, 3], vec![-4, -3, 6, 10, 20, 30], 3), 2);
    }
}

