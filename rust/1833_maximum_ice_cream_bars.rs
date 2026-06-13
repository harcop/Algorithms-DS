/// LeetCode #1833 - Maximum Ice Cream Bars
fn max_ice_cream(mut costs: Vec<i32>, mut coins: i32) -> i32 {
    costs.sort_unstable();
    let mut count = 0i32;
    for cost in costs {
        if coins < cost {
            break;
        }
        coins -= cost;
        count += 1;
    }
    count
}

fn main() {
    println!("{}", max_ice_cream(vec![1, 3, 2, 4, 1], 7));
}

#[cfg(test)]
mod tests {
    use super::max_ice_cream;

    #[test]
    fn example_one() {
        assert_eq!(max_ice_cream(vec![1, 3, 2, 4, 1], 7), 4);
    }

    #[test]
    fn example_two() {
        assert_eq!(max_ice_cream(vec![10, 6, 8, 7, 7, 8], 5), 0);
    }

    #[test]
    fn example_three() {
        assert_eq!(max_ice_cream(vec![1, 6, 3, 1, 2, 5], 20), 6);
    }
}
