/// LeetCode #1095 - Find in Mountain Array
pub struct MountainArray {
    arr: Vec<i32>,
}

impl MountainArray {
    fn new(arr: Vec<i32>) -> Self {
        MountainArray { arr }
    }

    fn get(&self, index: i32) -> i32 {
        self.arr[index as usize]
    }

    fn length(&self) -> i32 {
        self.arr.len() as i32
    }
}

fn find_in_mountain_array(target: i32, mountain_arr: &MountainArray) -> i32 {
    let n = mountain_arr.length();
    let mut lo = 0;
    let mut hi = n - 1;
    while lo < hi {
        let mid = (lo + hi) / 2;
        if mountain_arr.get(mid) < mountain_arr.get(mid + 1) {
            lo = mid + 1;
        } else {
            hi = mid;
        }
    }
    let peak = lo;
    let left = bin_search(mountain_arr, target, 0, peak, true);
    if left != -1 {
        return left;
    }
    bin_search(mountain_arr, target, peak + 1, n - 1, false)
}

fn bin_search(arr: &MountainArray, target: i32, mut lo: i32, mut hi: i32, asc: bool) -> i32 {
    while lo <= hi {
        let mid = (lo + hi) / 2;
        let v = arr.get(mid);
        if v == target {
            return mid;
        }
        if (asc && v < target) || (!asc && v > target) {
            lo = mid + 1;
        } else {
            hi = mid - 1;
        }
    }
    -1
}

fn main() {
    let m = MountainArray::new(vec![1, 2, 3, 4, 5, 3, 1]);
    println!("{}", find_in_mountain_array(3, &m));
}

#[cfg(test)]
mod tests {
    use super::{find_in_mountain_array, MountainArray};

    #[test]
    fn example_one() {
        let m = MountainArray::new(vec![1, 2, 3, 4, 5, 3, 1]);
        assert_eq!(find_in_mountain_array(3, &m), 2);
    }

    #[test]
    fn example_two() {
        let m = MountainArray::new(vec![0, 1, 2, 4, 2, 1]);
        assert_eq!(find_in_mountain_array(3, &m), -1);
    }
}
