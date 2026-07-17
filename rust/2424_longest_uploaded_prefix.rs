/// LeetCode #2424 - Longest Uploaded Prefix
struct LUPrefix {
    uploaded: Vec<bool>,
    longest: usize,
}

impl LUPrefix {
    fn new(n: i32) -> Self {
        Self {
            uploaded: vec![false; n as usize + 2],
            longest: 0,
        }
    }

    fn upload(&mut self, video: i32) {
        self.uploaded[video as usize] = true;
        while self.uploaded[self.longest + 1] {
            self.longest += 1;
        }
    }

    fn longest(&self) -> i32 {
        self.longest as i32
    }
}

fn main() {
    let mut prefix = LUPrefix::new(4);
    prefix.upload(3);
    prefix.upload(1);
    prefix.upload(2);
    println!("{}", prefix.longest());
}

#[cfg(test)]
mod tests {
    use super::LUPrefix;

    #[test]
    fn example_one() {
        let mut prefix = LUPrefix::new(4);
        prefix.upload(3);
        assert_eq!(prefix.longest(), 0);
        prefix.upload(1);
        assert_eq!(prefix.longest(), 1);
        prefix.upload(2);
        assert_eq!(prefix.longest(), 3);
    }

    #[test]
    fn uploads_in_order() {
        let mut prefix = LUPrefix::new(2);
        prefix.upload(1);
        prefix.upload(2);
        assert_eq!(prefix.longest(), 2);
    }
}
