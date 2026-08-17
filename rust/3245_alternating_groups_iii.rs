/// LeetCode #3245 - Alternating Groups III
use std::collections::BTreeSet;

struct Fenwick {
    bit: Vec<i64>,
}

impl Fenwick {
    fn new(n: usize) -> Self {
        Fenwick { bit: vec![0; n + 1] }
    }

    fn add(&mut self, mut i: usize, val: i64) {
        i += 1;
        while i < self.bit.len() {
            self.bit[i] += val;
            i += i & i.wrapping_neg();
        }
    }

    fn query(&self, mut i: usize) -> i64 {
        i += 1;
        let mut ret = 0;
        while i > 0 {
            ret += self.bit[i];
            i -= i & i.wrapping_neg();
        }
        ret
    }
}

fn update(
    i: i32,
    d: i32,
    n: i32,
    sl: &mut BTreeSet<i32>,
    bit1: &mut Fenwick,
    bit2: &mut Fenwick,
) {
    if d == 1 {
        sl.insert(i);
        if sl.len() == 1 {
            bit1.add(n as usize, 1);
            bit2.add(n as usize, n as i64);
        }
    }
    if sl.len() != 1 {
        let prv = sl
            .range(..i)
            .next_back()
            .copied()
            .or_else(|| sl.iter().next_back().copied())
            .unwrap();
        let nxt = sl
            .range((std::ops::Bound::Excluded(i), std::ops::Bound::Unbounded))
            .next()
            .copied()
            .or_else(|| sl.iter().next().copied())
            .unwrap();
        let mut l = ((nxt - prv + (n - 1)).rem_euclid(n)) + 1;
        bit1.add(l as usize, d as i64 * -1);
        bit2.add(l as usize, d as i64 * -(l as i64));
        l = (i - prv).rem_euclid(n);
        bit1.add(l as usize, d as i64);
        bit2.add(l as usize, d as i64 * l as i64);
        l = (nxt - i).rem_euclid(n);
        bit1.add(l as usize, d as i64);
        bit2.add(l as usize, d as i64 * l as i64);
    }
    if d == -1 {
        if sl.len() == 1 {
            bit1.add(n as usize, -1);
            bit2.add(n as usize, -(n as i64));
        }
        sl.remove(&i);
    }
}

fn number_of_alternating_groups(mut colors: Vec<i32>, queries: Vec<Vec<i32>>) -> Vec<i32> {
    let n = colors.len() as i32;
    let mut sl = BTreeSet::new();
    let mut bit1 = Fenwick::new((n + 1) as usize);
    let mut bit2 = Fenwick::new((n + 1) as usize);
    for i in 0..n {
        if colors[i as usize] == colors[((i + 1) % n) as usize] {
            update(i, 1, n, &mut sl, &mut bit1, &mut bit2);
        }
    }
    let mut result = Vec::new();
    for q in queries {
        if q[0] == 1 {
            let l = q[1];
            if sl.is_empty() {
                result.push(n);
            } else {
                let val = (bit2.query(n as usize) - bit2.query((l - 1) as usize))
                    - (l as i64 - 1) * (bit1.query(n as usize) - bit1.query((l - 1) as usize));
                result.push(val as i32);
            }
            continue;
        }
        let i = q[1];
        if colors[i as usize] == q[2] {
            continue;
        }
        colors[i as usize] = q[2];
        let left = (i - 1).rem_euclid(n);
        update(
            left,
            if colors[i as usize] == colors[left as usize] {
                1
            } else {
                -1
            },
            n,
            &mut sl,
            &mut bit1,
            &mut bit2,
        );
        update(
            i,
            if colors[i as usize] == colors[((i + 1) % n) as usize] {
                1
            } else {
                -1
            },
            n,
            &mut sl,
            &mut bit1,
            &mut bit2,
        );
    }
    result
}

fn main() {
    println!(
        "{:?}",
        number_of_alternating_groups(vec![0, 1, 1, 0, 1], vec![vec![2, 1, 0], vec![1, 4]])
    );
}

#[cfg(test)]
mod tests {
    use super::number_of_alternating_groups;

    #[test]
    fn example1() {
        assert_eq!(
            number_of_alternating_groups(vec![0, 1, 1, 0, 1], vec![vec![2, 1, 0], vec![1, 4]]),
            vec![2]
        );
    }

    #[test]
    fn example2() {
        assert_eq!(
            number_of_alternating_groups(
                vec![0, 0, 1, 0, 1, 1],
                vec![vec![1, 3], vec![2, 3, 0], vec![1, 5]]
            ),
            vec![2, 0]
        );
    }
}
