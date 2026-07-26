/// LeetCode #2691 - Immutability Helper (JS problem; Rust clone-and-mutate analogue)
use std::collections::BTreeMap;

#[derive(Clone, Debug, PartialEq)]
enum Json {
    Number(i64),
    Array(Vec<Json>),
    Object(BTreeMap<String, Json>),
}

struct ImmutableHelper {
    obj: Json,
}

impl ImmutableHelper {
    fn new(obj: Json) -> Self {
        ImmutableHelper { obj }
    }

    fn produce<F>(&self, mutator: F) -> Json
    where
        F: FnOnce(&mut Json),
    {
        let mut clone = self.obj.clone();
        mutator(&mut clone);
        clone
    }
}

fn main() {
    let helper = ImmutableHelper::new(Json::Object(BTreeMap::from([(
        "val".into(),
        Json::Number(10),
    )])));
    let next = helper.produce(|proxy| {
        if let Json::Object(map) = proxy {
            if let Some(Json::Number(v)) = map.get_mut("val") {
                *v += 1;
            }
        }
    });
    println!("{:?}", next);
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::BTreeMap;

    #[test]
    fn example_one() {
        let helper = ImmutableHelper::new(Json::Object(BTreeMap::from([(
            "val".into(),
            Json::Number(10),
        )])));
        let a = helper.produce(|proxy| {
            if let Json::Object(map) = proxy {
                if let Some(Json::Number(v)) = map.get_mut("val") {
                    *v += 1;
                }
            }
        });
        let b = helper.produce(|proxy| {
            if let Json::Object(map) = proxy {
                if let Some(Json::Number(v)) = map.get_mut("val") {
                    *v -= 1;
                }
            }
        });
        assert_eq!(
            a,
            Json::Object(BTreeMap::from([("val".into(), Json::Number(11))]))
        );
        assert_eq!(
            b,
            Json::Object(BTreeMap::from([("val".into(), Json::Number(9))]))
        );
        assert_eq!(
            helper.obj,
            Json::Object(BTreeMap::from([("val".into(), Json::Number(10))]))
        );
    }

    #[test]
    fn example_two() {
        let helper = ImmutableHelper::new(Json::Object(BTreeMap::from([(
            "arr".into(),
            Json::Array(vec![Json::Number(1), Json::Number(2), Json::Number(3)]),
        )])));
        let next = helper.produce(|proxy| {
            if let Json::Object(map) = proxy {
                if let Some(Json::Array(arr)) = map.get_mut("arr") {
                    arr[0] = Json::Number(5);
                    let new_val = match (&arr[0], &arr[1]) {
                        (Json::Number(a), Json::Number(b)) => a + b,
                        _ => 0,
                    };
                    map.insert("newVal".into(), Json::Number(new_val));
                }
            }
        });
        assert_eq!(
            next,
            Json::Object(BTreeMap::from([
                (
                    "arr".into(),
                    Json::Array(vec![Json::Number(5), Json::Number(2), Json::Number(3)])
                ),
                ("newVal".into(), Json::Number(7)),
            ]))
        );
    }
}
