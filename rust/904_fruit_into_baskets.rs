/// LeetCode #904 - Fruit Into Baskets
use std::collections::HashMap;

fn total_fruit(fruits: Vec<i32>) -> i32 {
    let mut cnt: HashMap<i32, i32> = HashMap::new();
    let mut j = 0usize;
    let mut best = 0usize;
    for i in 0..fruits.len() {
        *cnt.entry(fruits[i]).or_insert(0) += 1;
        while cnt.len() > 2 {
            let c = cnt.get_mut(&fruits[j]).unwrap();
            *c -= 1;
            if *c == 0 {
                cnt.remove(&fruits[j]);
            }
            j += 1;
        }
        best = best.max(i - j + 1);
    }
    best as i32
}

fn main() {
    println!("{}", total_fruit(vec![1, 2, 1]));
}

#[cfg(test)]
mod tests {
    use super::total_fruit;

    #[test]
    fn example_one() {
        assert_eq!(total_fruit(vec![1, 2, 1]), 3);
    }
}
