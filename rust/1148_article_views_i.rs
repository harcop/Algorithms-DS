/// LeetCode #1148 - Article Views I (SQL; Rust analogue)
use std::collections::HashSet;

fn article_views(views: Vec<(i32, i32, i32, String)>) -> Vec<i32> {
    let mut ids: HashSet<i32> = HashSet::new();
    for (_, author, viewer, _) in views {
        if author == viewer {
            ids.insert(author);
        }
    }
    let mut ans: Vec<i32> = ids.into_iter().collect();
    ans.sort();
    ans
}

fn main() {
    println!("ok");
}

#[cfg(test)]
mod tests {
    use super::article_views;

    #[test]
    fn example() {
        let views = vec![
            (1, 3, 5, "2019-08-01".into()),
            (1, 3, 6, "2019-08-02".into()),
            (2, 7, 7, "2019-08-01".into()),
            (2, 7, 6, "2019-08-02".into()),
            (4, 7, 1, "2019-07-22".into()),
            (3, 4, 4, "2019-07-21".into()),
            (3, 4, 4, "2019-07-21".into()),
        ];
        assert_eq!(article_views(views), vec![4, 7]);
    }
}
