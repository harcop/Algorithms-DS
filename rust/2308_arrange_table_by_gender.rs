/// LeetCode #2308 - Arrange Table by Gender (SQL; Rust analogue)
use std::collections::HashMap;

fn arrange_table_by_gender(genders: Vec<(i32, String)>) -> Vec<(i32, String)> {
    let mut buckets: HashMap<String, Vec<i32>> = HashMap::new();
    for (user_id, gender) in genders {
        buckets.entry(gender).or_default().push(user_id);
    }
    for v in buckets.values_mut() {
        v.sort();
    }
    let order = ["female", "other", "male"];
    let n = order
        .iter()
        .map(|g| buckets.get(*g).map(|v| v.len()).unwrap_or(0))
        .max()
        .unwrap_or(0);
    let mut ans = Vec::new();
    for i in 0..n {
        for g in order {
            if let Some(ids) = buckets.get(g) {
                if let Some(&id) = ids.get(i) {
                    ans.push((id, g.to_string()));
                }
            }
        }
    }
    ans
}

fn main() {
    println!("{:?}", arrange_table_by_gender(vec![]));
}

#[cfg(test)]
mod tests {
    use super::arrange_table_by_gender;

    #[test]
    fn example_one() {
        let genders = vec![
            (4, "male".into()),
            (7, "female".into()),
            (2, "other".into()),
            (5, "male".into()),
            (3, "female".into()),
            (8, "male".into()),
            (6, "other".into()),
            (1, "other".into()),
            (9, "female".into()),
        ];
        assert_eq!(
            arrange_table_by_gender(genders),
            vec![
                (3, "female".into()),
                (1, "other".into()),
                (4, "male".into()),
                (7, "female".into()),
                (2, "other".into()),
                (5, "male".into()),
                (9, "female".into()),
                (6, "other".into()),
                (8, "male".into()),
            ]
        );
    }
}
