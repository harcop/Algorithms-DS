/// LeetCode #3074 - Apple Redistribution into Boxes
fn minimum_boxes(apple: Vec<i32>, capacity: Vec<i32>) -> i32 {
    let total: i32 = apple.iter().sum();
    let mut caps = capacity;
    caps.sort_unstable_by(|a, b| b.cmp(a));
    let mut sum = 0;
    for (i, &c) in caps.iter().enumerate() {
        sum += c;
        if sum >= total {
            return (i + 1) as i32;
        }
    }
    caps.len() as i32
}

fn main() {
    println!("{}", minimum_boxes(vec![1, 3, 2], vec![4, 3, 1, 5, 2]));
}

#[cfg(test)]
mod tests {
    use super::minimum_boxes;

    #[test]
    fn example1() {
        assert_eq!(minimum_boxes(vec![1, 3, 2], vec![4, 3, 1, 5, 2]), 2);
    }

    #[test]
    fn example2() {
        assert_eq!(minimum_boxes(vec![5, 5, 5], vec![2, 4, 2, 7]), 4);
    }
}
