/// LeetCode #1435 - Create a Session Bar Chart (SQL; Rust analogue)
fn session_bar_chart(sessions: Vec<(i32, i32)>) -> Vec<(String, i32)> {
    let mut bins = [0i32; 4];
    for (_, duration) in sessions {
        if duration < 300 {
            bins[0] += 1;
        } else if duration < 600 {
            bins[1] += 1;
        } else if duration < 900 {
            bins[2] += 1;
        } else {
            bins[3] += 1;
        }
    }
    vec![
        ("[0-5>".into(), bins[0]),
        ("[5-10>".into(), bins[1]),
        ("[10-15>".into(), bins[2]),
        ("15 or more".into(), bins[3]),
    ]
}

fn main() {
    println!("{:?}", session_bar_chart(vec![]));
}

#[cfg(test)]
mod tests {
    use super::session_bar_chart;

    #[test]
    fn example() {
        let sessions = vec![(1, 30), (2, 199), (3, 299), (4, 580), (5, 1000)];
        assert_eq!(
            session_bar_chart(sessions),
            vec![
                ("[0-5>".into(), 3),
                ("[5-10>".into(), 1),
                ("[10-15>".into(), 0),
                ("15 or more".into(), 1),
            ]
        );
    }
}
