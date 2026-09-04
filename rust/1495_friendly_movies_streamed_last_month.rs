/// LeetCode #1495 - Friendly Movies Streamed Last Month (SQL; Rust analogue)
use std::collections::HashSet;

fn friendly_movies(
    tv: Vec<(String, i32, String)>,
    content: Vec<(i32, String, String, String)>,
) -> Vec<String> {
    let june: HashSet<i32> = tv
        .into_iter()
        .filter(|(d, _, _)| d.starts_with("2020-06"))
        .map(|(_, id, _)| id)
        .collect();
    let mut titles: Vec<String> = content
        .into_iter()
        .filter(|(id, _, kids, ty)| {
            june.contains(id) && kids == "Y" && ty == "Movies"
        })
        .map(|(_, title, _, _)| title)
        .collect();
    titles.sort();
    titles.dedup();
    titles
}

fn main() {
    println!("{:?}", friendly_movies(vec![], vec![]));
}

#[cfg(test)]
mod tests {
    use super::friendly_movies;

    #[test]
    fn example() {
        let tv = vec![
            ("2020-06-10 08:00".into(), 1, "LC-Channel".into()),
            ("2020-05-11 12:00".into(), 2, "LC-Channel".into()),
            ("2020-05-12 12:00".into(), 3, "LC-Channel".into()),
            ("2020-05-13 14:00".into(), 4, "Disney Ch".into()),
            ("2020-06-18 14:00".into(), 4, "Disney Ch".into()),
            ("2020-07-15 16:00".into(), 5, "Disney Ch".into()),
        ];
        let content = vec![
            (1, "Leetcode Movie".into(), "N".into(), "Movies".into()),
            (2, "Alg. for Kids".into(), "Y".into(), "Series".into()),
            (3, "Database Sols".into(), "N".into(), "Series".into()),
            (4, "Aladdin".into(), "Y".into(), "Movies".into()),
            (5, "Cinderella".into(), "Y".into(), "Movies".into()),
        ];
        assert_eq!(friendly_movies(tv, content), vec!["Aladdin".to_string()]);
    }
}
