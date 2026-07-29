/// LeetCode #2775 - Undefined to Null (JS problem; Rust analogue)
#[derive(Clone, Debug, PartialEq)]
enum Json {
    Null,
    Number(i64),
    Array(Vec<Json>),
    Object(Vec<(String, Json)>),
}

fn undefined_to_null(mut obj: Json) -> Json {
    match &mut obj {
        Json::Array(arr) => {
            for item in arr.iter_mut() {
                *item = undefined_to_null(std::mem::replace(item, Json::Null));
            }
        }
        Json::Object(pairs) => {
            for (_, val) in pairs.iter_mut() {
                *val = undefined_to_null(std::mem::replace(val, Json::Null));
            }
        }
        _ => {}
    }
    obj
}

fn main() {
    let obj = Json::Object(vec![
        ("a".into(), Json::Null),
        ("b".into(), Json::Number(3)),
    ]);
    println!("{:?}", undefined_to_null(obj));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_one() {
        let obj = Json::Object(vec![
            ("a".into(), Json::Null),
            ("b".into(), Json::Number(3)),
        ]);
        assert_eq!(
            undefined_to_null(obj),
            Json::Object(vec![
                ("a".into(), Json::Null),
                ("b".into(), Json::Number(3)),
            ])
        );
    }

    #[test]
    fn example_two() {
        let obj = Json::Object(vec![
            ("a".into(), Json::Null),
            ("b".into(), Json::Array(vec![Json::Number(1), Json::Null])),
        ]);
        assert_eq!(
            undefined_to_null(obj),
            Json::Object(vec![
                ("a".into(), Json::Null),
                ("b".into(), Json::Array(vec![Json::Number(1), Json::Null])),
            ])
        );
    }
}
