/// LeetCode #2298 - Tasks Count in the Weekend (SQL; Rust analogue)
fn weekday(y: i32, m: i32, d: i32) -> i32 {
    let t = [0, 3, 2, 5, 0, 3, 5, 1, 4, 6, 2, 4];
    let y = if m < 3 { y - 1 } else { y };
    (y + y / 4 - y / 100 + y / 400 + t[(m - 1) as usize] + d) % 7
}

fn tasks_count_in_the_weekend(tasks: Vec<(i32, i32, String)>) -> (i32, i32) {
    let mut weekend = 0;
    let mut working = 0;
    for (_tid, _aid, date) in tasks {
        let mut p = date.split('-');
        let y: i32 = p.next().unwrap().parse().unwrap();
        let m: i32 = p.next().unwrap().parse().unwrap();
        let d: i32 = p.next().unwrap().parse().unwrap();
        let wd = weekday(y, m, d);
        if wd == 0 || wd == 6 {
            weekend += 1;
        } else {
            working += 1;
        }
    }
    (weekend, working)
}

fn main() {
    println!("{:?}", tasks_count_in_the_weekend(vec![]));
}

#[cfg(test)]
mod tests {
    use super::tasks_count_in_the_weekend;

    #[test]
    fn example_one() {
        let tasks = vec![
            (1, 1, "2022-06-13".into()),
            (2, 6, "2022-06-14".into()),
            (3, 6, "2022-06-15".into()),
            (4, 3, "2022-06-18".into()),
            (5, 5, "2022-06-19".into()),
            (6, 7, "2022-06-19".into()),
        ];
        assert_eq!(tasks_count_in_the_weekend(tasks), (3, 3));
    }
}
