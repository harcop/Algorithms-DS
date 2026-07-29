/// LeetCode #2793 - Status of Flight Tickets (SQL problem; Rust analogue)
#[derive(Debug, PartialEq, Eq)]
struct TicketStatus {
    passenger_id: i32,
    status: String,
}

fn status_of_flight_tickets(
    flights: Vec<(i32, i32)>,
    passengers: Vec<(i32, i32, i32)>,
) -> Vec<TicketStatus> {
    use std::collections::HashMap;
    let capacity: HashMap<i32, i32> = flights.into_iter().collect();
    let mut by_flight: HashMap<i32, Vec<(i32, i32)>> = HashMap::new();
    for (passenger_id, flight_id, booking_time) in passengers {
        by_flight
            .entry(flight_id)
            .or_default()
            .push((passenger_id, booking_time));
    }
    let mut ans = Vec::new();
    for (flight_id, mut list) in by_flight {
        list.sort_by_key(|&(_, t)| t);
        let cap = capacity.get(&flight_id).copied().unwrap_or(0);
        for (rank, (passenger_id, _)) in list.into_iter().enumerate() {
            let status = if (rank as i32 + 1) <= cap {
                "Confirmed".to_string()
            } else {
                "Waitlist".to_string()
            };
            ans.push(TicketStatus {
                passenger_id,
                status,
            });
        }
    }
    ans.sort_by_key(|r| r.passenger_id);
    ans
}

fn main() {
    let flights = vec![(1, 2), (2, 2), (3, 1)];
    let passengers = vec![
        (101, 1, 1),
        (102, 1, 3),
        (103, 1, 2),
        (104, 2, 1),
        (105, 2, 2),
        (106, 3, 2),
        (107, 3, 1),
    ];
    println!("{:?}", status_of_flight_tickets(flights, passengers));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_one() {
        let flights = vec![(1, 2), (2, 2), (3, 1)];
        let passengers = vec![
            (101, 1, 1),
            (102, 1, 3),
            (103, 1, 2),
            (104, 2, 1),
            (105, 2, 2),
            (106, 3, 2),
            (107, 3, 1),
        ];
        assert_eq!(
            status_of_flight_tickets(flights, passengers),
            vec![
                TicketStatus {
                    passenger_id: 101,
                    status: "Confirmed".into(),
                },
                TicketStatus {
                    passenger_id: 102,
                    status: "Waitlist".into(),
                },
                TicketStatus {
                    passenger_id: 103,
                    status: "Confirmed".into(),
                },
                TicketStatus {
                    passenger_id: 104,
                    status: "Confirmed".into(),
                },
                TicketStatus {
                    passenger_id: 105,
                    status: "Confirmed".into(),
                },
                TicketStatus {
                    passenger_id: 106,
                    status: "Waitlist".into(),
                },
                TicketStatus {
                    passenger_id: 107,
                    status: "Confirmed".into(),
                },
            ]
        );
    }
}
