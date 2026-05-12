/// LeetCode #702 - Search in a Sorted Array of Unknown Size
pub struct ArrayReader {
    data: Vec<i32>,
}

impl ArrayReader {
    pub fn new(data: Vec<i32>) -> Self {
        Self { data }
    }

    pub fn get(&self, index: i32) -> i32 {
        if index < 0 || (index as usize) >= self.data.len() {
            return i32::MAX;
        }
        self.data[index as usize]
    }
}

fn search(reader: &ArrayReader, target: i32) -> i32 {
    let mut lo = 0i32;
    let mut hi = 1i32;
    while reader.get(hi) < target {
        lo = hi;
        hi *= 2;
    }
    while lo <= hi {
        let mid = lo + (hi - lo) / 2;
        let v = reader.get(mid);
        if v == target {
            return mid;
        }
        if v < target {
            lo = mid + 1;
        } else {
            hi = mid - 1;
        }
    }
    -1
}

fn main() {
    let r = ArrayReader::new(vec![-1, 0, 3, 5, 9, 12]);
    println!("{}", search(&r, 9));
}

#[cfg(test)]
mod tests {
    use super::{search, ArrayReader};

    #[test]
    fn example_one() {
        let r = ArrayReader::new(vec![-1, 0, 3, 5, 9, 12]);
        assert_eq!(search(&r, 9), 4);
    }

    #[test]
    fn example_two() {
        let r = ArrayReader::new(vec![-1, 0, 3, 5, 9, 12]);
        assert_eq!(search(&r, 2), -1);
    }
}
