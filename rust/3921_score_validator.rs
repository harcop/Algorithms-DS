/// LeetCode #3921 - Score Validator
fn score_validator(events: Vec<String>) -> Vec<i32> {
    let mut score = 0i32;
    let mut counter = 0i32;
    for event in events {
        match event.as_str() {
            "W" => counter += 1,
            "WD" | "NB" => score += 1,
            d => score += d.parse::<i32>().unwrap(),
        }
        if counter == 10 {
            break;
        }
    }
    vec![score, counter]
}

fn main() {
    println!(
        "{:?}",
        score_validator(vec![
            "1".into(),
            "4".into(),
            "W".into(),
            "6".into(),
            "WD".into()
        ])
    );
}

#[cfg(test)]
mod tests {
    use super::score_validator;

    #[test]
    fn example1() {
        assert_eq!(
            score_validator(vec![
                "1".into(),
                "4".into(),
                "W".into(),
                "6".into(),
                "WD".into()
            ]),
            vec![12, 1]
        );
    }

    #[test]
    fn example2() {
        assert_eq!(
            score_validator(vec![
                "WD".into(),
                "NB".into(),
                "0".into(),
                "4".into(),
                "4".into()
            ]),
            vec![10, 0]
        );
    }

    #[test]
    fn example3() {
        assert_eq!(
            score_validator(vec![
                "W".into(),
                "W".into(),
                "W".into(),
                "W".into(),
                "W".into(),
                "W".into(),
                "W".into(),
                "W".into(),
                "W".into(),
                "W".into(),
                "W".into()
            ]),
            vec![0, 10]
        );
    }
}
