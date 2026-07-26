/// LeetCode #2693 - Call Function with Custom Context (JS problem; Rust analogue)
use std::collections::BTreeMap;

#[derive(Clone, Debug)]
enum Json {
    Number(i64),
    String(String),
}

fn call_with_context<F, R>(ctx: &BTreeMap<String, Json>, args: &[Json], f: F) -> R
where
    F: FnOnce(&BTreeMap<String, Json>, &[Json]) -> R,
{
    f(ctx, args)
}

fn main() {
    let ctx = BTreeMap::from([("a".into(), Json::Number(5))]);
    let result = call_with_context(&ctx, &[Json::Number(7)], |this, args| {
        let a = match this.get("a") {
            Some(Json::Number(n)) => *n,
            _ => 0,
        };
        let b = match args.first() {
            Some(Json::Number(n)) => *n,
            _ => 0,
        };
        a + b
    });
    println!("{result}");
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::BTreeMap;

    #[test]
    fn example_one() {
        let ctx = BTreeMap::from([("a".into(), Json::Number(5))]);
        let result = call_with_context(&ctx, &[Json::Number(7)], |this, args| {
            let a = match this.get("a") {
                Some(Json::Number(n)) => *n,
                _ => 0,
            };
            let b = match args.first() {
                Some(Json::Number(n)) => *n,
                _ => 0,
            };
            a + b
        });
        assert_eq!(result, 12);
    }

    #[test]
    fn example_two() {
        let ctx = BTreeMap::from([("item".into(), Json::String("burger".into()))]);
        let result = call_with_context(
            &ctx,
            &[Json::Number(10), Json::Number(11) /* 10 * 1.1 as cents-like */],
            |this, args| {
                let item = match this.get("item") {
                    Some(Json::String(s)) => s.as_str(),
                    _ => "",
                };
                let price = match args.first() {
                    Some(Json::Number(n)) => *n,
                    _ => 0,
                };
                // use 11/10 as 1.1 analogue via integer: price * 11 / 10
                let total = price * 11 / 10;
                format!("The cost of the {item} is {total}")
            },
        );
        assert_eq!(result, "The cost of the burger is 11");
    }
}
