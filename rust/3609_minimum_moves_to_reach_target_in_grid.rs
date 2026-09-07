/// LeetCode #3609 - Minimum Moves to Reach Target in Grid
fn min_moves(sx: i32, sy: i32, mut tx: i32, mut ty: i32) -> i32 {
    let mut ans = 0;
    while (sx, sy) != (tx, ty) {
        if sx > tx || sy > ty {
            return -1;
        }
        if tx < ty {
            if tx > ty - tx {
                ty -= tx;
            } else {
                if ty % 2 != 0 {
                    return -1;
                }
                ty -= ty / 2;
            }
        } else if tx > ty {
            if ty > tx - ty {
                tx -= ty;
            } else {
                if tx % 2 != 0 {
                    return -1;
                }
                tx -= tx / 2;
            }
        } else if sx == 0 {
            tx -= ty;
        } else if sy == 0 {
            ty -= tx;
        } else {
            return -1;
        }
        ans += 1;
    }
    ans
}

fn main() {
    println!("{}", min_moves(1, 2, 5, 4));
}

#[cfg(test)]
mod tests {
    use super::min_moves;

    #[test]
    fn example1() {
        assert_eq!(min_moves(1, 2, 5, 4), 2);
    }

    #[test]
    fn example2() {
        assert_eq!(min_moves(0, 1, 2, 3), 3);
    }

    #[test]
    fn example3() {
        assert_eq!(min_moves(1, 1, 2, 2), -1);
    }

    #[test]
    fn same_point() {
        assert_eq!(min_moves(3, 4, 3, 4), 0);
    }
}
