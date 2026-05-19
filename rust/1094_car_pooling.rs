/// LeetCode #1094 - Car Pooling
fn car_pooling(trips: Vec<Vec<i32>>, capacity: i32) -> bool {
    let mut diff = vec![0i32; 1001];
    for t in trips {
        diff[t[1] as usize] += t[0];
        diff[t[2] as usize] -= t[0];
    }
    let mut cur = 0;
    for x in diff {
        cur += x;
        if cur > capacity {
            return false;
        }
    }
    true
}

fn main() {
    println!(
        "{}",
        car_pooling(vec![vec![2, 1, 5], vec![3, 3, 7]], 4)
    );
}

#[cfg(test)]
mod tests {
    use super::car_pooling;

    #[test]
    fn example_one() {
        assert!(car_pooling(vec![vec![2, 1, 5], vec![3, 3, 7]], 5));
    }

    #[test]
    fn example_two() {
        assert!(!car_pooling(vec![vec![2, 1, 5], vec![3, 3, 7]], 4));
    }
}
