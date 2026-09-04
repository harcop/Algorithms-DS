/// LeetCode #1279 - Traffic Light Controlled Intersection
struct TrafficLight {
    green_road: i32,
}

impl TrafficLight {
    fn new() -> Self {
        TrafficLight { green_road: 1 }
    }

    fn car_arrived(&mut self, car_id: i32, road_id: i32) -> (i32, bool) {
        let turned = if self.green_road != road_id {
            self.green_road = road_id;
            true
        } else {
            false
        };
        (car_id, turned)
    }
}

fn main() {
    let mut tl = TrafficLight::new();
    println!("{:?}", tl.car_arrived(1, 1));
}

#[cfg(test)]
mod tests {
    use super::TrafficLight;

    #[test]
    fn example() {
        let mut tl = TrafficLight::new();
        // cars: id 1 road A, id 3 road A, id 5 road A, id 2 road B, id 4 road B
        assert_eq!(tl.car_arrived(1, 1), (1, false));
        assert_eq!(tl.car_arrived(3, 1), (3, false));
        assert_eq!(tl.car_arrived(5, 1), (5, false));
        assert_eq!(tl.car_arrived(2, 2), (2, true));
        assert_eq!(tl.car_arrived(4, 2), (4, false));
    }
}
