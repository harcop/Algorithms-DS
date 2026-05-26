/// LeetCode #1396 - Design Underground System
use std::collections::HashMap;

struct UndergroundSystem {
    checkins: HashMap<i32, (String, i32)>,
    routes: HashMap<(String, String), (i64, i32)>,
}

impl UndergroundSystem {
    fn new() -> Self {
        Self {
            checkins: HashMap::new(),
            routes: HashMap::new(),
        }
    }

    fn check_in(&mut self, id: i32, station_name: String, t: i32) {
        self.checkins.insert(id, (station_name, t));
    }

    fn check_out(&mut self, id: i32, station_name: String, t: i32) {
        let (start, t0) = self.checkins.remove(&id).unwrap();
        let key = (start, station_name);
        let entry = self.routes.entry(key).or_insert((0, 0));
        entry.0 += (t - t0) as i64;
        entry.1 += 1;
    }

    fn get_average_time(&self, start_station: String, end_station: String) -> f64 {
        let (total, cnt) = self.routes[&(start_station, end_station)];
        total as f64 / cnt as f64
    }
}

fn main() {
    let mut s = UndergroundSystem::new();
    s.check_in(45, "Leyton".into(), 3);
    s.check_in(32, "Paradise".into(), 8);
    s.check_out(45, "Paradise".into(), 15);
    println!("{}", s.get_average_time("Leyton".into(), "Paradise".into()));
}

#[cfg(test)]
mod tests {
    use super::UndergroundSystem;

    #[test]
    fn example_one() {
        let mut s = UndergroundSystem::new();
        s.check_in(45, "Leyton".into(), 3);
        s.check_in(32, "Paradise".into(), 8);
        s.check_out(45, "Paradise".into(), 15);
        assert!((s.get_average_time("Leyton".into(), "Paradise".into()) - 12.0).abs() < 1e-9);
    }
}

