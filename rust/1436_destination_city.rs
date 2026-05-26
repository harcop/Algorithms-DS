/// LeetCode #1436 - Destination City
fn dest_city(paths: Vec<Vec<String>>) -> String {
    use std::collections::HashSet;
    let starts: HashSet<_> = paths.iter().map(|p| p[0].clone()).collect();
    for p in paths {
        if !starts.contains(&p[1]) {
            return p[1].clone();
        }
    }
    String::new()
}

fn main() {
    println!("{}", dest_city(vec![vec!["London".into(), "New York".into()], vec!["New York".into(), "Lima".into()], vec!["Lima".into(), "Sao Paulo".into()]]));
}

#[cfg(test)]
mod tests {
    use super::dest_city;

    #[test]
    fn example_one() {
        assert_eq!(
            dest_city(vec![
                vec!["London".into(), "New York".into()],
                vec!["New York".into(), "Lima".into()],
                vec!["Lima".into(), "Sao Paulo".into()],
            ]),
            "Sao Paulo"
        );
    }
}

