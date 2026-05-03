/// LeetCode #218 - The Skyline Problem
use std::collections::BTreeMap;

fn get_skyline(buildings: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    let mut events: Vec<(i32, i32)> = vec![];
    for b in buildings {
        events.push((b[0], -b[2]));
        events.push((b[1], b[2]));
    }
    events.sort_by(|a, b| a.0.cmp(&b.0).then_with(|| a.1.cmp(&b.1)));
    let mut cnt: BTreeMap<i32, i32> = BTreeMap::new();
    *cnt.entry(0).or_insert(0) += 1;
    let mut prev = 0;
    let mut out: Vec<Vec<i32>> = vec![];
    let mut i = 0usize;
    while i < events.len() {
        let x = events[i].0;
        while i < events.len() && events[i].0 == x {
            let h = events[i].1;
            if h < 0 {
                *cnt.entry(-h).or_insert(0) += 1;
            } else {
                let e = cnt.get_mut(&h).unwrap();
                *e -= 1;
                if *e == 0 {
                    cnt.remove(&h);
                }
            }
            i += 1;
        }
        let cur = *cnt.iter().rev().next().unwrap().0;
        if cur != prev {
            out.push(vec![x, cur]);
            prev = cur;
        }
    }
    out
}

fn main() {
    println!(
        "{:?}",
        get_skyline(vec![vec![2, 9, 10], vec![3, 7, 15], vec![5, 12, 12], vec![15, 20, 10], vec![19, 24, 8]])
    );
}

#[cfg(test)]
mod tests {
    use super::get_skyline;

    #[test]
    fn example_one() {
        let b = vec![
            vec![2, 9, 10],
            vec![3, 7, 15],
            vec![5, 12, 12],
            vec![15, 20, 10],
            vec![19, 24, 8],
        ];
        let exp = vec![
            vec![2, 10],
            vec![3, 15],
            vec![7, 12],
            vec![12, 0],
            vec![15, 10],
            vec![20, 8],
            vec![24, 0],
        ];
        assert_eq!(get_skyline(b), exp);
    }
}
