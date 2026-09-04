/// LeetCode #2314 - The First Day of the Maximum Recorded Degree in Each City (SQL; Rust analogue)
use std::collections::HashMap;

fn first_day_max_degree(weather: Vec<(i32, String, i32)>) -> Vec<(i32, String, i32)> {
    let mut best: HashMap<i32, (String, i32)> = HashMap::new();
    for (city_id, day, degree) in weather {
        best.entry(city_id)
            .and_modify(|(bd, bdeg)| {
                if degree > *bdeg || (degree == *bdeg && day < *bd) {
                    *bd = day.clone();
                    *bdeg = degree;
                }
            })
            .or_insert((day, degree));
    }
    let mut ans: Vec<(i32, String, i32)> = best
        .into_iter()
        .map(|(city, (day, deg))| (city, day, deg))
        .collect();
    ans.sort_by_key(|t| t.0);
    ans
}

fn main() {
    println!("{:?}", first_day_max_degree(vec![]));
}

#[cfg(test)]
mod tests {
    use super::first_day_max_degree;

    #[test]
    fn example_one() {
        let weather = vec![
            (1, "2022-01-07".into(), -12),
            (1, "2022-03-07".into(), 5),
            (1, "2022-07-07".into(), 24),
            (2, "2022-08-07".into(), 37),
            (2, "2022-08-17".into(), 37),
            (3, "2022-02-07".into(), -7),
            (3, "2022-12-07".into(), -6),
        ];
        assert_eq!(
            first_day_max_degree(weather),
            vec![
                (1, "2022-07-07".into(), 24),
                (2, "2022-08-07".into(), 37),
                (3, "2022-12-07".into(), -6),
            ]
        );
    }
}
