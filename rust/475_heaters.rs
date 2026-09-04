/// LeetCode #475 - Heaters
fn find_radius(mut houses: Vec<i32>, mut heaters: Vec<i32>) -> i32 {
    houses.sort_unstable();
    heaters.sort_unstable();
    let mut radius = 0i32;
    for h in houses {
        let pos = heaters.binary_search(&h).unwrap_or_else(|e| e);
        let mut dist = i32::MAX;
        if pos < heaters.len() {
            dist = dist.min((heaters[pos] - h).abs());
        }
        if pos > 0 {
            dist = dist.min((heaters[pos - 1] - h).abs());
        }
        radius = radius.max(dist);
    }
    radius
}

fn main() {
    println!("{}", find_radius(vec![1, 2, 3], vec![2]));
}

#[cfg(test)]
mod tests {
    use super::find_radius;

    #[test]
    fn example_one() {
        assert_eq!(find_radius(vec![1, 2, 3], vec![2]), 1);
    }

    #[test]
    fn example_two() {
        assert_eq!(find_radius(vec![1, 2, 3, 4], vec![1, 4]), 1);
    }

    #[test]
    fn example_three() {
        assert_eq!(find_radius(vec![1, 5], vec![2]), 3);
    }
}
