/// LeetCode #1860 - Incremental Memory Leak
fn mem_leak(mut memory1: i32, mut memory2: i32) -> Vec<i32> {
    let mut i = 1;
    while i <= memory1.max(memory2) {
        if memory1 >= memory2 {
            memory1 -= i;
        } else {
            memory2 -= i;
        }
        i += 1;
    }
    vec![i, memory1, memory2]
}

fn main() {
    println!("{:?}", mem_leak(2, 2));
}

#[cfg(test)]
mod tests {
    use super::mem_leak;

    #[test]
    fn example_one() {
        assert_eq!(mem_leak(2, 2), vec![3, 1, 0]);
    }

    #[test]
    fn example_two() {
        assert_eq!(mem_leak(8, 11), vec![6, 0, 4]);
    }
}
