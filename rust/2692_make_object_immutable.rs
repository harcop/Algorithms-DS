/// LeetCode #2692 - Make Object Immutable (JS problem; Rust Result-based analogue)
use std::collections::BTreeMap;

#[derive(Clone, Debug, PartialEq)]
enum Json {
    Number(i64),
    Array(Vec<Json>),
    Object(BTreeMap<String, Json>),
}

#[derive(Debug, PartialEq)]
enum ImmError {
    ModifyingKey(String),
    ModifyingIndex(usize),
    CallingMethod(String),
}

impl ImmError {
    fn message(&self) -> String {
        match self {
            ImmError::ModifyingKey(k) => format!("Error Modifying: {k}"),
            ImmError::ModifyingIndex(i) => format!("Error Modifying Index: {i}"),
            ImmError::CallingMethod(m) => format!("Error Calling Method: {m}"),
        }
    }
}

struct ImmutableJson {
    inner: Json,
}

impl ImmutableJson {
    fn new(obj: Json) -> Self {
        ImmutableJson { inner: obj }
    }

    fn get_object_keys(&self) -> Result<Vec<String>, ImmError> {
        match &self.inner {
            Json::Object(map) => Ok(map.keys().cloned().collect()),
            _ => Ok(vec![]),
        }
    }

    fn set_key(&mut self, key: &str, _value: Json) -> Result<(), ImmError> {
        Err(ImmError::ModifyingKey(key.to_string()))
    }

    fn set_index(&mut self, index: usize, _value: Json) -> Result<(), ImmError> {
        Err(ImmError::ModifyingIndex(index))
    }

    fn call_mutating_method(&mut self, method: &str) -> Result<(), ImmError> {
        const MUTATING: &[&str] = &["pop", "push", "shift", "unshift", "splice", "sort", "reverse"];
        if MUTATING.contains(&method) {
            Err(ImmError::CallingMethod(method.to_string()))
        } else {
            Ok(())
        }
    }
}

fn main() {
    let mut obj = ImmutableJson::new(Json::Object(BTreeMap::from([(
        "x".into(),
        Json::Number(5),
    )])));
    println!("{:?}", obj.set_key("x", Json::Number(5)).err().map(|e| e.message()));
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::BTreeMap;

    #[test]
    fn example_one() {
        let mut obj = ImmutableJson::new(Json::Object(BTreeMap::from([(
            "x".into(),
            Json::Number(5),
        )])));
        let err = obj.set_key("x", Json::Number(5)).unwrap_err();
        assert_eq!(err.message(), "Error Modifying: x");
    }

    #[test]
    fn example_two() {
        let mut obj = ImmutableJson::new(Json::Array(vec![
            Json::Number(1),
            Json::Number(2),
            Json::Number(3),
        ]));
        let err = obj.set_index(1, Json::Object(BTreeMap::new())).unwrap_err();
        assert_eq!(err.message(), "Error Modifying Index: 1");
    }

    #[test]
    fn example_three() {
        let mut obj = ImmutableJson::new(Json::Object(BTreeMap::from([(
            "arr".into(),
            Json::Array(vec![Json::Number(1), Json::Number(2), Json::Number(3)]),
        )])));
        let err = obj.call_mutating_method("push").unwrap_err();
        assert_eq!(err.message(), "Error Calling Method: push");
    }

    #[test]
    fn example_four() {
        let obj = ImmutableJson::new(Json::Object(BTreeMap::from([
            ("x".into(), Json::Number(2)),
            ("y".into(), Json::Number(2)),
        ])));
        assert_eq!(
            obj.get_object_keys().unwrap(),
            vec!["x".to_string(), "y".to_string()]
        );
    }
}
