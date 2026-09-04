/// LeetCode #1341 - Movie Rating (SQL; Rust analogue)
use std::collections::HashMap;

fn movie_rating(
    movies: Vec<(i32, String)>,
    users: Vec<(i32, String)>,
    ratings: Vec<(i32, i32, i32, String)>,
) -> Vec<String> {
    let user_name: HashMap<i32, String> = users.into_iter().collect();
    let movie_title: HashMap<i32, String> = movies.into_iter().collect();
    let mut user_cnt: HashMap<i32, i32> = HashMap::new();
    let mut feb: HashMap<i32, (i32, i32)> = HashMap::new();
    for (mid, uid, rating, date) in ratings {
        *user_cnt.entry(uid).or_insert(0) += 1;
        if date.starts_with("2020-02") {
            let e = feb.entry(mid).or_insert((0, 0));
            e.0 += rating;
            e.1 += 1;
        }
    }
    let best_user = user_cnt
        .into_iter()
        .map(|(uid, c)| (c, user_name[&uid].clone()))
        .max_by(|a, b| a.0.cmp(&b.0).then(b.1.cmp(&a.1)))
        .unwrap()
        .1;
    let best_movie = feb
        .into_iter()
        .map(|(mid, (sum, n))| (sum as f64 / n as f64, movie_title[&mid].clone()))
        .max_by(|a, b| a.0.partial_cmp(&b.0).unwrap().then(b.1.cmp(&a.1)))
        .unwrap()
        .1;
    vec![best_user, best_movie]
}

fn main() {
    println!("{:?}", movie_rating(vec![], vec![], vec![]));
}

#[cfg(test)]
mod tests {
    use super::movie_rating;

    #[test]
    fn example() {
        let movies = vec![
            (1, "Avengers".into()),
            (2, "Frozen 2".into()),
            (3, "Joker".into()),
        ];
        let users = vec![
            (1, "Daniel".into()),
            (2, "Monica".into()),
            (3, "Maria".into()),
            (4, "James".into()),
        ];
        let ratings = vec![
            (1, 1, 3, "2020-01-12".into()),
            (1, 2, 4, "2020-02-11".into()),
            (1, 3, 2, "2020-02-12".into()),
            (1, 4, 1, "2020-01-01".into()),
            (2, 1, 5, "2020-02-17".into()),
            (2, 2, 2, "2020-02-01".into()),
            (2, 3, 2, "2020-03-01".into()),
            (3, 1, 3, "2020-02-22".into()),
            (3, 2, 4, "2020-02-25".into()),
        ];
        assert_eq!(
            movie_rating(movies, users, ratings),
            vec!["Daniel".to_string(), "Frozen 2".to_string()]
        );
    }
}
