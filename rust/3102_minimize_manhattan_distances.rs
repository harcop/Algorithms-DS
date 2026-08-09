/// LeetCode #3102 - Minimize Manhattan Distances
use std::collections::BTreeMap;

fn merge(map: &mut BTreeMap<i32, i32>, key: i32, delta: i32) {
    let e = map.entry(key).or_insert(0);
    *e += delta;
    if *e == 0 {
        map.remove(&key);
    }
}

fn minimum_distance(points: Vec<Vec<i32>>) -> i32 {
    let mut tm1: BTreeMap<i32, i32> = BTreeMap::new();
    let mut tm2: BTreeMap<i32, i32> = BTreeMap::new();
    for p in &points {
        let (x, y) = (p[0], p[1]);
        merge(&mut tm1, x + y, 1);
        merge(&mut tm2, x - y, 1);
    }
    let mut ans = i32::MAX;
    for p in &points {
        let (x, y) = (p[0], p[1]);
        merge(&mut tm1, x + y, -1);
        merge(&mut tm2, x - y, -1);
        let d1 = tm1.keys().next_back().unwrap() - tm1.keys().next().unwrap();
        let d2 = tm2.keys().next_back().unwrap() - tm2.keys().next().unwrap();
        ans = ans.min(d1.max(d2));
        merge(&mut tm1, x + y, 1);
        merge(&mut tm2, x - y, 1);
    }
    ans
}

fn main() {
    println!(
        "{}",
        minimum_distance(vec![
            vec![3, 10],
            vec![5, 15],
            vec![10, 2],
            vec![4, 4]
        ])
    );
}

#[cfg(test)]
mod tests {
    use super::minimum_distance;

    #[test]
    fn example1() {
        assert_eq!(
            minimum_distance(vec![
                vec![3, 10],
                vec![5, 15],
                vec![10, 2],
                vec![4, 4]
            ]),
            12
        );
    }

    #[test]
    fn example2() {
        assert_eq!(
            minimum_distance(vec![vec![1, 1], vec![1, 1], vec![1, 1]]),
            0
        );
    }
}
