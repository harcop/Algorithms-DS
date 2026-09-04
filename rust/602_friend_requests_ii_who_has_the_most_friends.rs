/// LeetCode #602 - Friend Requests II: Who Has the Most Friends (SQL; Rust analogue)
use std::collections::HashMap;

fn most_friends(request_accepted: Vec<(i32, i32, String)>) -> (i32, i32) {
    let mut cnt: HashMap<i32, i32> = HashMap::new();
    for (a, b, _) in request_accepted {
        *cnt.entry(a).or_insert(0) += 1;
        *cnt.entry(b).or_insert(0) += 1;
    }
    cnt.into_iter().max_by_key(|(id, n)| (*n, -id)).map(|(id, n)| (id, n)).unwrap()
}

fn main() {
    println!("ok");
}

#[cfg(test)]
mod tests {
    use super::most_friends;

    #[test]
    fn example() {
        let req = vec![
            (1, 2, "2016-06-03".into()),
            (1, 3, "2016-06-08".into()),
            (2, 3, "2016-06-08".into()),
            (3, 4, "2016-06-09".into()),
        ];
        assert_eq!(most_friends(req), (3, 3));
    }
}
