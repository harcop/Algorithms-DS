/// LeetCode #1603 - Design Parking System
pub struct ParkingSystem {
    big: i32,
    medium: i32,
    small: i32,
}

impl ParkingSystem {
    fn new(big: i32, medium: i32, small: i32) -> Self {
        ParkingSystem { big, medium, small }
    }
    fn add_car(&mut self, car_type: i32) -> bool {
        match car_type {
            1 => { if self.big > 0 { self.big -= 1; true } else { false } }
            2 => { if self.medium > 0 { self.medium -= 1; true } else { false } }
            _ => { if self.small > 0 { self.small -= 1; true } else { false } }
        }
    }
}
fn main() {
    let mut ps = ParkingSystem::new(1, 1, 0);
    println!("{}", ps.add_car(1));
}
#[cfg(test)]
mod tests {
    use super::ParkingSystem;
    #[test]
    fn example_one() {
        let mut ps = ParkingSystem::new(1, 1, 0);
        assert!(ps.add_car(1));
        assert!(ps.add_car(2));
        assert!(!ps.add_car(1));
        assert!(!ps.add_car(1));
    }
}