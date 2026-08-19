/// LeetCode #3308 - Find Top Performing Driver (SQL; Rust analogue)
use std::collections::BTreeMap;

fn find_top_performing_driver(
    drivers: Vec<(i32, i32)>,                 // (driver_id, accidents)
    vehicles: Vec<(i32, i32, String)>,        // (vehicle_id, driver_id, fuel_type)
    trips: Vec<(i32, i32, i32)>,              // (vehicle_id, distance, rating)
) -> Vec<(String, i32, i32, i32)> {
    // rating is hundredths (e.g. 450 => 4.50)
    let acc: BTreeMap<i32, i32> = drivers.into_iter().collect();
    let mut veh: BTreeMap<i32, (i32, String)> = BTreeMap::new();
    for (vid, did, fuel) in vehicles {
        veh.insert(vid, (did, fuel));
    }
    #[derive(Default)]
    struct Agg {
        rating_sum: i64,
        trips: i64,
        distance: i64,
        accidents: i32,
    }
    let mut agg: BTreeMap<(String, i32), Agg> = BTreeMap::new();
    for (vid, dist, rating) in trips {
        let (did, fuel) = veh.get(&vid).unwrap();
        let e = agg.entry((fuel.clone(), *did)).or_default();
        e.rating_sum += rating as i64;
        e.trips += 1;
        e.distance += dist as i64;
        e.accidents = acc[did];
    }
    let mut best: BTreeMap<String, (i32, i32, i64, i32)> = BTreeMap::new();
    // (driver_id, rating_hundredths, distance, accidents)
    for ((fuel, did), a) in agg {
        let rating = ((a.rating_sum * 100) as f64 / a.trips as f64).round() as i32;
        let cand = (did, rating, a.distance, a.accidents);
        best.entry(fuel)
            .and_modify(|cur| {
                if cand.1 > cur.1
                    || (cand.1 == cur.1 && cand.2 > cur.2)
                    || (cand.1 == cur.1 && cand.2 == cur.2 && cand.3 < cur.3)
                {
                    *cur = cand;
                }
            })
            .or_insert(cand);
    }
    best.into_iter()
        .map(|(fuel, (did, rating, dist, _))| (fuel, did, rating, dist as i32))
        .collect()
}

fn main() {
    let drivers = vec![(1, 1), (2, 3), (3, 0)];
    let vehicles = vec![
        (100, 1, "Gasoline".into()),
        (101, 2, "Electric".into()),
        (102, 3, "Gasoline".into()),
    ];
    let trips = vec![
        (100, 50, 5),
        (100, 30, 4),
        (101, 100, 4),
        (101, 80, 5),
        (102, 40, 5),
        (102, 60, 5),
    ];
    println!("{:?}", find_top_performing_driver(drivers, vehicles, trips));
}

#[cfg(test)]
mod tests {
    use super::find_top_performing_driver;

    #[test]
    fn example() {
        let drivers = vec![(1, 1), (2, 3), (3, 0)];
        let vehicles = vec![
            (100, 1, "Gasoline".into()),
            (101, 2, "Electric".into()),
            (102, 3, "Gasoline".into()),
        ];
        let trips = vec![
            (100, 50, 5),
            (100, 30, 4),
            (101, 100, 4),
            (101, 80, 5),
            (102, 40, 5),
            (102, 60, 5),
        ];
        assert_eq!(
            find_top_performing_driver(drivers, vehicles, trips),
            vec![
                ("Electric".into(), 2, 450, 180),
                ("Gasoline".into(), 3, 500, 100),
            ]
        );
    }
}
