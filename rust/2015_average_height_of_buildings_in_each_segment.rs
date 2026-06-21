/// LeetCode #2015 - Average Height of Buildings in Each Segment
use std::collections::HashMap;

fn average_height_of_buildings(buildings: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    let mut cnt = HashMap::new();
    let mut height = HashMap::new();
    for b in buildings {
        let (start, end, h) = (b[0], b[1], b[2]);
        *cnt.entry(start).or_insert(0) += 1;
        *cnt.entry(end).or_insert(0) -= 1;
        *height.entry(start).or_insert(0) += h;
        *height.entry(end).or_insert(0) -= h;
    }

    let mut keys: Vec<i32> = cnt.keys().copied().collect();
    keys.sort_unstable();

    let mut ans: Vec<Vec<i32>> = Vec::new();
    let mut s = 0i64;
    let mut m = 0i32;
    let mut last = -1;
    for k in keys {
        if m > 0 {
            let avg = (s / m as i64) as i32;
            if let Some(seg) = ans.last_mut() {
                if seg[2] == avg && seg[1] == last {
                    seg[1] = k;
                } else {
                    ans.push(vec![last, k, avg]);
                }
            } else {
                ans.push(vec![last, k, avg]);
            }
        }
        s += height.get(&k).copied().unwrap_or(0) as i64;
        m += cnt.get(&k).copied().unwrap_or(0);
        last = k;
    }
    ans
}

fn main() {
    println!(
        "{:?}",
        average_height_of_buildings(vec![vec![1, 4, 2], vec![3, 9, 4]])
    );
}

#[cfg(test)]
mod tests {
    use super::average_height_of_buildings;

    #[test]
    fn example_one() {
        assert_eq!(
            average_height_of_buildings(vec![vec![1, 4, 2], vec![3, 9, 4]]),
            vec![vec![1, 3, 2], vec![3, 4, 3], vec![4, 9, 4]]
        );
    }

    #[test]
    fn example_two() {
        assert_eq!(
            average_height_of_buildings(vec![vec![1, 3, 2], vec![2, 5, 3], vec![2, 8, 3]]),
            vec![vec![1, 3, 2], vec![3, 8, 3]]
        );
    }

    #[test]
    fn example_three() {
        assert_eq!(
            average_height_of_buildings(vec![vec![1, 2, 1], vec![5, 6, 1]]),
            vec![vec![1, 2, 1], vec![5, 6, 1]]
        );
    }
}
