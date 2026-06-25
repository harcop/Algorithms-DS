/// LeetCode #2080 - Range Frequency Queries
use std::collections::HashMap;

pub struct RangeFreqQuery {
    positions: HashMap<i32, Vec<i32>>,
}

impl RangeFreqQuery {
    fn new(arr: Vec<i32>) -> Self {
        let mut positions: HashMap<i32, Vec<i32>> = HashMap::new();
        for (i, value) in arr.into_iter().enumerate() {
            positions.entry(value).or_default().push(i as i32);
        }
        RangeFreqQuery { positions }
    }

    fn query(&self, left: i32, right: i32, value: i32) -> i32 {
        let Some(indices) = self.positions.get(&value) else {
            return 0;
        };
        let lo = indices.partition_point(|&i| i < left);
        let hi = indices.partition_point(|&i| i <= right);
        (hi - lo) as i32
    }
}

fn main() {
    let rfq = RangeFreqQuery::new(vec![12, 33, 4, 56, 22, 2, 34, 33, 22, 12, 34, 56]);
    println!("{}", rfq.query(1, 2, 4));
}

#[cfg(test)]
mod tests {
    use super::RangeFreqQuery;

    #[test]
    fn example_sequence() {
        let rfq = RangeFreqQuery::new(vec![12, 33, 4, 56, 22, 2, 34, 33, 22, 12, 34, 56]);
        assert_eq!(rfq.query(1, 2, 4), 1);
        assert_eq!(rfq.query(0, 11, 33), 2);
    }

    #[test]
    fn missing_value() {
        let rfq = RangeFreqQuery::new(vec![1, 1, 2, 2]);
        assert_eq!(rfq.query(0, 3, 3), 0);
    }
}
