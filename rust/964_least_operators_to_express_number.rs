/// LeetCode #964 - Least Operators to Express Number

fn least_ops_express_target(x: i32, target: i32) -> i32 {
    let x = x as i64;
    let mut target = target as i64;
    let mut pos = 0i32;
    let mut neg = 0i32;
    while target > 0 {
        let rem = target % x;
        target /= x;
        if rem == 0 {
            pos += 1;
        } else {
            let cost_add = (rem * 2 + pos as i64 + neg as i64) as i32;
            let cost_sub = ((x - rem) * 2 + pos as i64 + neg as i64 + 1) as i32;
            if cost_add < cost_sub {
                pos += rem as i32;
            } else {
                neg += (x - rem) as i32;
                target += 1;
            }
        }
    }
    pos + neg - 1
}

fn main() {
    println!("{}", least_ops_express_target(3, 19));
}

#[cfg(test)]
mod tests {
    use super::least_ops_express_target;

    #[test]
    fn example_one() {
        assert_eq!(least_ops_express_target(3, 19), 5);
    }

    #[test]
    fn example_two() {
        assert_eq!(least_ops_express_target(5, 501), 8);
    }

    #[test]
    fn example_three() {
        assert_eq!(least_ops_express_target(100, 100000000), 3);
    }
}
