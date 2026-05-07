/// LeetCode #379 - Design Phone Directory (`used` bitmask / vector)
struct PhoneDirectory {
    used: Vec<bool>,
}

impl PhoneDirectory {
    fn new(max_numbers: i32) -> Self {
        PhoneDirectory {
            used: vec![false; max_numbers as usize],
        }
    }

    fn get(&mut self) -> i32 {
        for i in 0..self.used.len() {
            if !self.used[i] {
                self.used[i] = true;
                return i as i32;
            }
        }
        -1
    }

    fn check(&self, number: i32) -> bool {
        let i = number as usize;
        i < self.used.len() && !self.used[i]
    }

    fn release(&mut self, number: i32) {
        let i = number as usize;
        if i < self.used.len() {
            self.used[i] = false;
        }
    }
}

fn main() {
    let mut p = PhoneDirectory::new(3);
    println!("{}", p.get());
}

#[cfg(test)]
mod tests {
    use super::PhoneDirectory;

    #[test]
    fn lc_flow() {
        let mut d = PhoneDirectory::new(3);
        assert_eq!(d.get(), 0);
        assert!(!d.check(0));
        d.release(0);
        assert!(d.check(0));
        assert_eq!(d.get(), 0);
        assert!(d.check(2));
        assert_eq!(d.get(), 1);
        assert_eq!(d.get(), 2);
        assert_eq!(d.get(), -1);
        d.release(2);
        assert_eq!(d.get(), 2);
    }
}
