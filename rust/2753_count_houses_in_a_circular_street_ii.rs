/// LeetCode #2753 - Count Houses in a Circular Street II
struct Street {
    doors: Vec<bool>,
    pos: usize,
}

impl Street {
    fn new(doors: Vec<bool>) -> Self {
        Street { doors, pos: 0 }
    }
    fn close_door(&mut self) {
        self.doors[self.pos] = false;
    }
    fn is_door_open(&self) -> bool {
        self.doors[self.pos]
    }
    fn move_right(&mut self) {
        self.pos = (self.pos + 1) % self.doors.len();
    }
}

fn house_count(street: &mut Street, k: i32) -> i32 {
    while !street.is_door_open() {
        street.move_right();
    }
    let mut ans = 0;
    for i in 1..=k {
        street.move_right();
        if street.is_door_open() {
            ans = i;
            street.close_door();
        }
    }
    ans
}

fn main() {
    let mut s = Street::new(vec![true, true, true, true]);
    println!("{}", house_count(&mut s, 10));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_one() {
        let mut s = Street::new(vec![true, true, true, true]);
        assert_eq!(house_count(&mut s, 10), 4);
    }

    #[test]
    fn example_two() {
        let mut s = Street::new(vec![true, false, true, true, false]);
        assert_eq!(house_count(&mut s, 5), 5);
    }
}
