/// LeetCode #780 - Reaching Points
fn reaching_points(sx: i32, sy: i32, tx: i32, ty: i32) -> bool {
    let (mut tx, mut ty) = (tx as i64, ty as i64);
    let (sx, sy) = (sx as i64, sy as i64);
    while tx >= sx && ty >= sy {
        if tx == sx && ty == sy {
            return true;
        }
        if tx > ty {
            if ty < sy {
                return false;
            }
            if ty == sy {
                return (tx - sx) % (ty - sy + 1) == 0;
            }
            tx -= ((tx - sx) / (ty - sy)).max(1) * (ty - sy);
        } else {
            if tx < sx {
                return false;
            }
            if tx == sx {
                return (ty - sy) % (tx - sx + 1) == 0;
            }
            ty -= ((ty - sy) / (tx - sx)).max(1) * (tx - sx);
        }
    }
    false
}

fn main() {
    println!("{}", reaching_points(1, 1, 3, 5));
}

#[cfg(test)]
mod tests {
    use super::reaching_points;

    #[test]
    fn example_one() {
        assert!(reaching_points(1, 1, 3, 5));
    }

    #[test]
    fn example_two() {
        assert!(!reaching_points(9, 5, 12, 9));
    }
}
