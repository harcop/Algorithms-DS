/// LeetCode #1533 - Find The Index Of The Large Integer
pub trait ArrayReader {
    fn compare_sub(&self, l: i32, r: i32, x: i32, y: i32) -> i32;
}

struct Reader<'a> {
    arr: &'a [i32],
}

impl ArrayReader for Reader<'_> {
    fn compare_sub(&self, l: i32, r: i32, x: i32, y: i32) -> i32 {
        let l = l as usize;
        let r = r as usize;
        let x = x as usize;
        let y = y as usize;
        let len = r - l + 1;
        for i in 0..len {
            if self.arr[l + i] < self.arr[x + i] {
                return -1;
            }
            if self.arr[l + i] > self.arr[x + i] {
                return 1;
            }
        }
        0
    }
}

fn get_index(reader: &dyn ArrayReader, arr: Vec<i32>) -> i32 {
    let mut left = 0i32;
    let mut right = arr.len() as i32 - 1;
    while left < right {
        let mid = left + (right - left) / 2;
        if reader.compare_sub(mid, mid, mid + 1, mid + 1) < 0 {
            right = mid;
        } else {
            left = mid + 1;
        }
    }
    if reader.compare_sub(left, left, left + 1, left + 1) < 0 {
        left + 1
    } else {
        left
    }
}

fn main() {
    let arr = vec![7, 7, 7, 7, 10, 10, 10];
    let reader = Reader { arr: &arr };
    println!("{}", get_index(&reader, arr.clone()));
}

#[cfg(test)]
mod tests {
    use super::{get_index, Reader};

    #[test]
    fn example_one() {
        let arr = vec![7, 7, 7, 7, 10, 10, 10];
        let reader = Reader { arr: &arr };
        assert_eq!(get_index(&reader, arr.clone()), 4);
    }
}
