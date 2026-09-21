/// LeetCode #3861 - Minimum Capacity Box
fn minimum_index(capacity: Vec<i32>, item_size: i32) -> i32 {
    let mut ans = -1;
    for (i, &x) in capacity.iter().enumerate() {
        if x >= item_size && (ans == -1 || x < capacity[ans as usize]) {
            ans = i as i32;
        }
    }
    ans
}

fn main() {
    println!("{}", minimum_index(vec![1, 5, 3, 7], 3));
}

#[cfg(test)]
mod tests {
    use super::minimum_index;

    #[test]
    fn example1() {
        assert_eq!(minimum_index(vec![1, 5, 3, 7], 3), 2);
    }

    #[test]
    fn example2() {
        assert_eq!(minimum_index(vec![3, 5, 4, 3], 2), 0);
    }

    #[test]
    fn example3() {
        assert_eq!(minimum_index(vec![4], 5), -1);
    }
}
