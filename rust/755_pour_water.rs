/// LeetCode #755 - Pour Water
fn pour_water(heights: Vec<i32>, volume: i32, k: i32) -> Vec<i32> {
    let mut h = heights;
    let k = k as usize;
    for _ in 0..volume {
        let mut best = k;
        let mut i = k;
        while i > 0 && h[i - 1] <= h[i] {
            if h[i - 1] < h[best] {
                best = i - 1;
            }
            i -= 1;
        }
        if h[best] < h[k] {
            h[best] += 1;
            continue;
        }
        let mut best = k;
        let mut i = k;
        while i + 1 < h.len() && h[i + 1] <= h[i] {
            if h[i + 1] < h[best] {
                best = i + 1;
            }
            i += 1;
        }
        h[best] += 1;
    }
    h
}

fn main() {
    println!("{:?}", pour_water(vec![2, 1, 1, 2, 1, 2], 4, 3));
}

#[cfg(test)]
mod tests {
    use super::pour_water;

    #[test]
    fn example_one() {
        assert_eq!(pour_water(vec![2, 1, 1, 2, 1, 2], 4, 3), vec![2, 2, 2, 3, 2, 2]);
    }
}
