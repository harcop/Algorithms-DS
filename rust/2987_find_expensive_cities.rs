/// LeetCode #2987 - Find Expensive Cities (SQL; Rust analogue)
use std::collections::HashMap;

fn find_expensive_cities(listings: Vec<(i32, String, i32)>) -> Vec<String> {
    // (listing_id, city, price)
    let national: f64 =
        listings.iter().map(|(_, _, p)| *p as f64).sum::<f64>() / listings.len() as f64;
    let mut sum: HashMap<String, (f64, i32)> = HashMap::new();
    for (_, city, price) in listings {
        let e = sum.entry(city).or_insert((0.0, 0));
        e.0 += price as f64;
        e.1 += 1;
    }
    let mut ans: Vec<_> = sum
        .into_iter()
        .filter(|(_, (s, c))| s / (*c as f64) > national)
        .map(|(city, _)| city)
        .collect();
    ans.sort();
    ans
}

fn main() {
    let listings = vec![
        (113, "LosAngeles".into(), 7560386),
        (136, "SanFrancisco".into(), 2380268),
        (92, "Chicago".into(), 9833209),
        (60, "Chicago".into(), 5147582),
        (8, "Chicago".into(), 5274441),
        (79, "SanFrancisco".into(), 8372065),
        (37, "Chicago".into(), 7939595),
        (53, "LosAngeles".into(), 4965123),
        (178, "SanFrancisco".into(), 999207),
        (51, "NewYork".into(), 5951718),
        (121, "NewYork".into(), 2893760),
    ];
    println!("{:?}", find_expensive_cities(listings));
}

#[cfg(test)]
mod tests {
    use super::find_expensive_cities;

    #[test]
    fn example() {
        let listings = vec![
            (113, "LosAngeles".into(), 7560386),
            (136, "SanFrancisco".into(), 2380268),
            (92, "Chicago".into(), 9833209),
            (60, "Chicago".into(), 5147582),
            (8, "Chicago".into(), 5274441),
            (79, "SanFrancisco".into(), 8372065),
            (37, "Chicago".into(), 7939595),
            (53, "LosAngeles".into(), 4965123),
            (178, "SanFrancisco".into(), 999207),
            (51, "NewYork".into(), 5951718),
            (121, "NewYork".into(), 2893760),
        ];
        assert_eq!(
            find_expensive_cities(listings),
            vec!["Chicago".to_string(), "LosAngeles".to_string()]
        );
    }
}
