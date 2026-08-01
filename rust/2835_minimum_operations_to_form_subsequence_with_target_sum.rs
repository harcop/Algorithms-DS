/// LeetCode #2835 - Minimum Operations to Form Subsequence With Target Sum
fn min_operations(nums: Vec<i32>, target: i32) -> i32 {
    if nums.iter().map(|&x| x as i64).sum::<i64>() < target as i64 {
        return -1;
    }

    let mut count = [0i32; 33];
    for x in nums {
        for (bit, value) in count.iter_mut().enumerate().take(32) {
            if (x >> bit) & 1 == 1 {
                *value += 1;
            }
        }
    }

    let mut bit = 0usize;
    let mut available = 0usize;
    let mut operations = 0;
    loop {
        while bit < 32 && ((target >> bit) & 1) == 0 {
            bit += 1;
        }
        if bit == 32 {
            break;
        }
        while available < bit {
            count[available + 1] += count[available] / 2;
            count[available] %= 2;
            available += 1;
        }
        while count[available] == 0 {
            count[available] = 1;
            available += 1;
        }
        operations += (available - bit) as i32;
        count[available] -= 1;
        available = bit;
        bit += 1;
    }
    operations
}

fn main() {
    println!("{}", min_operations(vec![1, 2, 8], 7));
}

#[cfg(test)]
mod tests {
    use super::min_operations;

    #[test]
    fn examples() {
        assert_eq!(min_operations(vec![1, 2, 8], 7), 1);
        assert_eq!(min_operations(vec![1, 32, 1, 2], 12), 2);
        assert_eq!(min_operations(vec![1, 32, 1], 35), -1);
    }
}
