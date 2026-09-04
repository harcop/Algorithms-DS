/// LeetCode #1715 - Count Apples and Oranges (SQL; Rust analogue)
use std::collections::HashMap;

fn count_apples_oranges(
    boxes: Vec<(i32, Option<i32>, i32, i32)>,
    chests: Vec<(i32, i32, i32)>,
) -> (i32, i32) {
    let chest: HashMap<i32, (i32, i32)> = chests.into_iter().map(|(id, a, o)| (id, (a, o))).collect();
    let mut apples = 0;
    let mut oranges = 0;
    for (_, cid, a, o) in boxes {
        apples += a;
        oranges += o;
        if let Some(id) = cid {
            if let Some(&(ca, co)) = chest.get(&id) {
                apples += ca;
                oranges += co;
            }
        }
    }
    (apples, oranges)
}

fn main() {
    println!("{:?}", count_apples_oranges(vec![], vec![]));
}

#[cfg(test)]
mod tests {
    use super::count_apples_oranges;

    #[test]
    fn example() {
        let boxes = vec![
            (2, None, 6, 15),
            (18, Some(14), 4, 15),
            (19, Some(3), 8, 4),
            (12, Some(2), 19, 20),
            (20, Some(6), 12, 9),
            (8, Some(6), 9, 9),
            (3, Some(14), 16, 7),
        ];
        let chests = vec![
            (6, 5, 6),
            (14, 20, 10),
            (2, 8, 8),
            (3, 19, 4),
            (16, 19, 19),
        ];
        assert_eq!(count_apples_oranges(boxes, chests), (151, 123));
    }
}
