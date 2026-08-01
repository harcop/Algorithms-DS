use std::collections::HashMap;

/// LeetCode #2837 - Total Traveled Distance
fn total_traveled_distance(
    users: Vec<(i32, String)>,
    rides: Vec<(i32, i32, i32)>,
) -> Vec<(i32, String, i64)> {
    let mut distance_by_user = HashMap::<i32, i64>::new();
    for (_, user_id, distance) in rides {
        *distance_by_user.entry(user_id).or_default() += distance as i64;
    }

    let mut result: Vec<_> = users
        .into_iter()
        .map(|(user_id, name)| {
            let distance = distance_by_user.get(&user_id).copied().unwrap_or(0);
            (user_id, name, distance)
        })
        .collect();
    result.sort_unstable_by_key(|row| row.0);
    result
}

fn main() {
    let users = vec![(17, "Addison".into()), (14, "Ethan".into())];
    let rides = vec![(72, 17, 160), (42, 14, 161)];
    println!("{:?}", total_traveled_distance(users, rides));
}

#[cfg(test)]
mod tests {
    use super::total_traveled_distance;

    #[test]
    fn example() {
        let users = vec![
            (17, "Addison".into()),
            (14, "Ethan".into()),
            (4, "Michael".into()),
            (2, "Avery".into()),
            (10, "Eleanor".into()),
        ];
        let rides = vec![
            (72, 17, 160),
            (42, 14, 161),
            (45, 4, 59),
            (32, 2, 197),
            (15, 4, 357),
            (56, 2, 196),
            (10, 14, 25),
        ];
        assert_eq!(
            total_traveled_distance(users, rides),
            vec![
                (2, "Avery".into(), 393),
                (4, "Michael".into(), 416),
                (10, "Eleanor".into(), 0),
                (14, "Ethan".into(), 186),
                (17, "Addison".into(), 160),
            ]
        );
    }
}
