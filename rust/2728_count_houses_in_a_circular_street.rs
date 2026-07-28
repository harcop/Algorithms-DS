/// LeetCode #2728 - Count Houses in a Circular Street
struct Street {
    doors: Vec<bool>,
    pos: usize,
}

impl Street {
    fn new(doors: Vec<i32>) -> Self {
        Street {
            doors: doors.into_iter().map(|d| d != 0).collect(),
            pos: 0,
        }
    }

    fn open_door(&mut self) {
        self.doors[self.pos] = true;
    }

    fn close_door(&mut self) {
        self.doors[self.pos] = false;
    }

    fn is_door_open(&self) -> bool {
        self.doors[self.pos]
    }

    #[allow(dead_code)]
    fn move_right(&mut self) {
        self.pos = (self.pos + 1) % self.doors.len();
    }

    fn move_left(&mut self) {
        self.pos = (self.pos + self.doors.len() - 1) % self.doors.len();
    }
}

fn house_count(street: &mut Street, mut k: i32) -> i32 {
    while k > 0 {
        street.open_door();
        street.move_left();
        k -= 1;
    }
    let mut ans = 0;
    while street.is_door_open() {
        ans += 1;
        street.close_door();
        street.move_left();
    }
    ans
}

fn main() {
    let mut street = Street::new(vec![0, 0, 0, 0]);
    println!("{}", house_count(&mut street, 10));
}

#[cfg(test)]
mod tests {
    use super::{house_count, Street};

    #[test]
    fn example_one() {
        let mut street = Street::new(vec![0, 0, 0, 0]);
        assert_eq!(house_count(&mut street, 10), 4);
    }

    #[test]
    fn example_two() {
        let mut street = Street::new(vec![1, 0, 1, 1, 0]);
        assert_eq!(house_count(&mut street, 5), 5);
    }
}
