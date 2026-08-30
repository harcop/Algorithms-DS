/// LeetCode #3477 - Fruits Into Baskets II
fn num_of_unplaced_fruits(fruits: Vec<i32>, baskets: Vec<i32>) -> i32 {
    let n = fruits.len();
    let mut vis = vec![false; n];
    let mut ans = n as i32;
    for &x in &fruits {
        for (i, &y) in baskets.iter().enumerate() {
            if y >= x && !vis[i] {
                vis[i] = true;
                ans -= 1;
                break;
            }
        }
    }
    ans
}

fn main() {
    println!("{}", num_of_unplaced_fruits(vec![4, 2, 5], vec![3, 5, 4]));
}

#[cfg(test)]
mod tests {
    use super::num_of_unplaced_fruits;

    #[test]
    fn example1() {
        assert_eq!(num_of_unplaced_fruits(vec![4, 2, 5], vec![3, 5, 4]), 1);
    }

    #[test]
    fn example2() {
        assert_eq!(num_of_unplaced_fruits(vec![3, 6, 1], vec![6, 4, 7]), 0);
    }
}
