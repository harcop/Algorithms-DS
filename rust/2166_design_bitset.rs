/// LeetCode #2166 - Design Bitset
pub struct Bitset {
    bits: Vec<u8>,
    flipped: u8,
    ones: i32,
}

impl Bitset {
    fn new(size: i32) -> Self {
        Bitset {
            bits: vec![0; size as usize],
            flipped: 0,
            ones: 0,
        }
    }

    fn fix(&mut self, idx: i32) {
        let i = idx as usize;
        if self.bits[i] ^ self.flipped == 0 {
            self.bits[i] = 1 ^ self.flipped;
            self.ones += 1;
        }
    }

    fn unfix(&mut self, idx: i32) {
        let i = idx as usize;
        if self.bits[i] ^ self.flipped == 1 {
            self.bits[i] = self.flipped;
            self.ones -= 1;
        }
    }

    fn flip(&mut self) {
        self.flipped ^= 1;
        self.ones = self.bits.len() as i32 - self.ones;
    }

    fn all(&self) -> bool {
        self.ones == self.bits.len() as i32
    }

    fn one(&self) -> bool {
        self.ones > 0
    }

    fn count(&self) -> i32 {
        self.ones
    }

    fn to_string(&self) -> String {
        self.bits
            .iter()
            .map(|&b| char::from(b'0' + (b ^ self.flipped)))
            .collect()
    }
}

fn main() {
    let mut bitset = Bitset::new(5);
    bitset.fix(3);
    println!("{}", bitset.to_string());
}

#[cfg(test)]
mod tests {
    use super::Bitset;

    #[test]
    fn example() {
        let mut bitset = Bitset::new(5);
        bitset.fix(3);
        bitset.fix(1);
        assert_eq!(bitset.to_string(), "01010");
        bitset.flip();
        assert_eq!(bitset.to_string(), "10101");
        assert!(!bitset.all());
        bitset.unfix(0);
        assert_eq!(bitset.to_string(), "00101");
        bitset.flip();
        assert_eq!(bitset.to_string(), "11010");
        assert!(bitset.one());
        bitset.unfix(0);
        assert_eq!(bitset.count(), 2);
        assert_eq!(bitset.to_string(), "01010");
    }
}
