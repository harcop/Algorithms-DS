/// LeetCode #2446 - Determine if Two Events Have Conflict
fn have_conflict(event1: Vec<String>, event2: Vec<String>) -> bool {
    event1[0] <= event2[1] && event2[0] <= event1[1]
}

fn main() {
    println!(
        "{}",
        have_conflict(
            vec!["01:15".to_string(), "02:00".to_string()],
            vec!["02:00".to_string(), "03:00".to_string()]
        )
    );
}

#[cfg(test)]
mod tests {
    use super::have_conflict;

    #[test]
    fn touching_events_conflict() {
        assert!(have_conflict(
            vec!["01:15".to_string(), "02:00".to_string()],
            vec!["02:00".to_string(), "03:00".to_string()]
        ));
    }

    #[test]
    fn separate_events() {
        assert!(!have_conflict(
            vec!["10:00".to_string(), "11:00".to_string()],
            vec!["14:00".to_string(), "15:00".to_string()]
        ));
    }
}
