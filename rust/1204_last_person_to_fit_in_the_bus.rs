/// LeetCode #1204 - Last Person to Fit in the Bus (SQL; Rust analogue)
fn last_person_to_fit(mut queue: Vec<(i32, String, i32, i32)>) -> String {
    queue.sort_by_key(|q| q.3);
    let mut sum = 0;
    let mut last = String::new();
    for (_, name, w, _) in queue {
        if sum + w > 1000 {
            break;
        }
        sum += w;
        last = name;
    }
    last
}

fn main() {
    println!("ok");
}

#[cfg(test)]
mod tests {
    use super::last_person_to_fit;

    #[test]
    fn example() {
        let queue = vec![
            (5, "Alice".into(), 250, 1),
            (4, "Bob".into(), 175, 5),
            (3, "Alex".into(), 350, 2),
            (6, "John Cena".into(), 400, 3),
            (1, "Winston".into(), 500, 6),
            (2, "Marie".into(), 200, 4),
        ];
        assert_eq!(last_person_to_fit(queue), "John Cena");
    }
}
