/// LeetCode #2783 - Flight Occupancy and Waitlist Analysis (SQL problem; Rust analogue)
#[derive(Debug, PartialEq, Eq)]
struct FlightResult {
    flight_id: i32,
    booked_cnt: i32,
    waitlist_cnt: i32,
}

fn flight_occupancy(
    flights: Vec<(i32, i32)>,
    passengers: Vec<(i32, i32)>,
) -> Vec<FlightResult> {
    use std::collections::HashMap;
    let mut counts: HashMap<i32, i32> = HashMap::new();
    for (_, flight_id) in passengers {
        *counts.entry(flight_id).or_insert(0) += 1;
    }
    let mut ans: Vec<FlightResult> = flights
        .into_iter()
        .map(|(flight_id, capacity)| {
            let total = counts.get(&flight_id).copied().unwrap_or(0);
            FlightResult {
                flight_id,
                booked_cnt: total.min(capacity),
                waitlist_cnt: (total - capacity).max(0),
            }
        })
        .collect();
    ans.sort_by_key(|r| r.flight_id);
    ans
}

fn main() {
    let flights = vec![(1, 2), (2, 2), (3, 1)];
    let passengers = vec![
        (101, 1),
        (102, 1),
        (103, 1),
        (104, 2),
        (105, 2),
        (106, 3),
        (107, 3),
    ];
    println!("{:?}", flight_occupancy(flights, passengers));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_one() {
        let flights = vec![(1, 2), (2, 2), (3, 1)];
        let passengers = vec![
            (101, 1),
            (102, 1),
            (103, 1),
            (104, 2),
            (105, 2),
            (106, 3),
            (107, 3),
        ];
        assert_eq!(
            flight_occupancy(flights, passengers),
            vec![
                FlightResult {
                    flight_id: 1,
                    booked_cnt: 2,
                    waitlist_cnt: 1,
                },
                FlightResult {
                    flight_id: 2,
                    booked_cnt: 2,
                    waitlist_cnt: 0,
                },
                FlightResult {
                    flight_id: 3,
                    booked_cnt: 1,
                    waitlist_cnt: 1,
                },
            ]
        );
    }
}
