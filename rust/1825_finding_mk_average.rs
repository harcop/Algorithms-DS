/// LeetCode #1825 - Finding MK Average
use std::collections::{BTreeMap, VecDeque};

fn multiset_len(map: &BTreeMap<i32, usize>) -> usize {
    map.values().sum()
}

fn multiset_add(map: &mut BTreeMap<i32, usize>, x: i32) {
    *map.entry(x).or_insert(0) += 1;
}

fn multiset_remove(map: &mut BTreeMap<i32, usize>, x: i32) -> bool {
    let e = map.get_mut(&x).unwrap();
    *e -= 1;
    if *e == 0 {
        map.remove(&x);
    }
    true
}

fn multiset_pop_max(map: &mut BTreeMap<i32, usize>) -> i32 {
    let k = *map.keys().next_back().unwrap();
    multiset_remove(map, k);
    k
}

fn multiset_pop_min(map: &mut BTreeMap<i32, usize>) -> i32 {
    let k = *map.keys().next().unwrap();
    multiset_remove(map, k);
    k
}

pub struct MKAverage {
    m: usize,
    k: usize,
    q: VecDeque<i32>,
    lo: BTreeMap<i32, usize>,
    mid: BTreeMap<i32, usize>,
    hi: BTreeMap<i32, usize>,
    mid_sum: i64,
}

impl MKAverage {
    fn new(m: i32, k: i32) -> Self {
        MKAverage {
            m: m as usize,
            k: k as usize,
            q: VecDeque::new(),
            lo: BTreeMap::new(),
            mid: BTreeMap::new(),
            hi: BTreeMap::new(),
            mid_sum: 0,
        }
    }

    fn rebalance(&mut self) {
        while multiset_len(&self.lo) > self.k {
            let x = multiset_pop_max(&mut self.lo);
            multiset_add(&mut self.mid, x);
            self.mid_sum += x as i64;
        }
        while multiset_len(&self.hi) > self.k {
            let x = multiset_pop_min(&mut self.hi);
            multiset_add(&mut self.mid, x);
            self.mid_sum += x as i64;
        }
        while multiset_len(&self.lo) < self.k && !self.mid.is_empty() {
            let x = multiset_pop_min(&mut self.mid);
            self.mid_sum -= x as i64;
            multiset_add(&mut self.lo, x);
        }
        while multiset_len(&self.hi) < self.k && !self.mid.is_empty() {
            let x = multiset_pop_max(&mut self.mid);
            self.mid_sum -= x as i64;
            multiset_add(&mut self.hi, x);
        }
    }

    fn add_element(&mut self, num: i32) {
        if self.lo.is_empty() || num <= *self.lo.keys().next_back().unwrap() {
            multiset_add(&mut self.lo, num);
        } else if self.hi.is_empty() || num >= *self.hi.keys().next().unwrap() {
            multiset_add(&mut self.hi, num);
        } else {
            multiset_add(&mut self.mid, num);
            self.mid_sum += num as i64;
        }
        self.q.push_back(num);
        if self.q.len() > self.m {
            let x = self.q.pop_front().unwrap();
            if self.lo.contains_key(&x) {
                multiset_remove(&mut self.lo, x);
            } else if self.hi.contains_key(&x) {
                multiset_remove(&mut self.hi, x);
            } else {
                multiset_remove(&mut self.mid, x);
                self.mid_sum -= x as i64;
            }
        }
        self.rebalance();
    }

    fn calculate_mk_average(&self) -> i32 {
        if self.q.len() < self.m {
            -1
        } else {
            (self.mid_sum / (self.m - 2 * self.k) as i64) as i32
        }
    }
}

fn main() {
    let mut obj = MKAverage::new(3, 1);
    obj.add_element(3);
    obj.add_element(1);
    println!("{}", obj.calculate_mk_average());
}

#[cfg(test)]
mod tests {
    use super::MKAverage;

    #[test]
    fn example_one() {
        let mut obj = MKAverage::new(3, 1);
        obj.add_element(3);
        obj.add_element(1);
        assert_eq!(obj.calculate_mk_average(), -1);
        obj.add_element(10);
        assert_eq!(obj.calculate_mk_average(), 3);
        obj.add_element(5);
        obj.add_element(5);
        obj.add_element(5);
        assert_eq!(obj.calculate_mk_average(), 5);
    }
}
