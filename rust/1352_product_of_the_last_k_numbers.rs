/// LeetCode #1352 - Product Of The Last K Numbers

struct ProductOfNumbers {
    nums: Vec<i32>,
}

impl ProductOfNumbers {
    fn new() -> Self {
        Self { nums: vec![] }
    }

    fn add(&mut self, num: i32) {
        self.nums.push(num);
    }

    fn get_product(&self, k: i32) -> i32 {
        let k = k as usize;
        let n = self.nums.len();
        if k > n {
            return 0;
        }
        self.nums[n - k..].iter().product()
    }
}

fn main() {
    let mut p = ProductOfNumbers::new();
    p.add(3);
    p.add(0);
    p.add(8);
    println!("{}", p.get_product(2));
}

#[cfg(test)]
mod tests {
    use super::ProductOfNumbers;

    #[test]
    fn example_one() {
        let mut p = ProductOfNumbers::new();
        p.add(3);
        p.add(0);
        p.add(8);
        assert_eq!(p.get_product(2), 0);
        assert_eq!(p.get_product(1), 8);
        p.add(2);
        assert_eq!(p.get_product(3), 0);
    }
}
