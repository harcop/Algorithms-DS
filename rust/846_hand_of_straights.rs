/// LeetCode #846 - Hand of Straights
fn is_n_straight_hand(hand: Vec<i32>, group_size: i32) -> bool {
    use std::collections::BTreeMap;
    let gs = group_size as i32;
    let mut cnt: BTreeMap<i32, usize> = BTreeMap::new();
    for c in hand {
        *cnt.entry(c).or_insert(0) += 1;
    }
    while let Some((&k, _)) = cnt.iter().next() {
        for x in k..k + gs {
            let e = cnt.get_mut(&x);
            if e.is_none() || *e.unwrap() == 0 {
                return false;
            }
            *cnt.get_mut(&x).unwrap() -= 1;
            if *cnt.get(&x).unwrap() == 0 {
                cnt.remove(&x);
            }
        }
    }
    true
}

fn main() {
    println!("{}", is_n_straight_hand(vec![1, 2, 3, 6, 2, 3, 4, 7, 8], 3));
}

#[cfg(test)]
mod tests {
    use super::is_n_straight_hand;

    #[test]
    fn example_one() {
        assert!(is_n_straight_hand(vec![1, 2, 3, 6, 2, 3, 4, 7, 8], 3));
    }

    #[test]
    fn example_two() {
        assert!(!is_n_straight_hand(vec![1, 2, 3, 4, 5], 4));
    }
}
