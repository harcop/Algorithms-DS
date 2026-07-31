/// LeetCode #2823 - Deep Object Filter (JS problem; Rust analogue)
#[derive(Debug, Clone, PartialEq, Eq)]
enum Json {
    Num(i64),
    Arr(Vec<Json>),
}

fn deep_filter(value: Json, predicate: fn(i64) -> bool) -> Option<Json> {
    match value {
        Json::Num(n) => {
            if predicate(n) {
                Some(Json::Num(n))
            } else {
                None
            }
        }
        Json::Arr(arr) => {
            let filtered: Vec<Json> = arr
                .into_iter()
                .filter_map(|v| deep_filter(v, predicate))
                .collect();
            if filtered.is_empty() {
                None
            } else {
                Some(Json::Arr(filtered))
            }
        }
    }
}

fn main() {
    let flat = Json::Arr(vec![
        Json::Num(-5),
        Json::Num(-4),
        Json::Num(-3),
        Json::Num(-2),
        Json::Num(-1),
        Json::Num(0),
        Json::Num(1),
    ]);
    println!("{:?}", deep_filter(flat, |x| x > 0));
}

#[cfg(test)]
mod tests {
    use super::{deep_filter, Json};

    #[test]
    fn example_one() {
        let flat = Json::Arr(vec![
            Json::Num(-5),
            Json::Num(-4),
            Json::Num(-3),
            Json::Num(-2),
            Json::Num(-1),
            Json::Num(0),
            Json::Num(1),
        ]);
        assert_eq!(
            deep_filter(flat, |x| x > 0),
            Some(Json::Arr(vec![Json::Num(1)]))
        );
    }

    #[test]
    fn example_two() {
        let nested = Json::Arr(vec![
            Json::Num(-1),
            Json::Arr(vec![
                Json::Num(-1),
                Json::Num(-1),
                Json::Num(5),
                Json::Num(-1),
                Json::Num(10),
            ]),
            Json::Num(-1),
            Json::Arr(vec![Json::Num(-1)]),
            Json::Arr(vec![Json::Num(-5)]),
        ]);
        assert_eq!(
            deep_filter(nested, |x| x > 0),
            Some(Json::Arr(vec![Json::Arr(vec![Json::Num(5), Json::Num(10)])]))
        );
    }
}
