/// LeetCode #2833 - Furthest Point From Origin
fn furthest_distance_from_origin(moves: String) -> i32 {
    let l = moves.chars().filter(|&c| c == 'L').count() as i32;
    let r = moves.chars().filter(|&c| c == 'R').count() as i32;
    let blank = moves.chars().filter(|&c| c == '_').count() as i32;
    (l - r).abs() + blank
}

fn main() {
    println!("{}", furthest_distance_from_origin("L_RL__R".into()));
}

#[cfg(test)]
mod tests {
    use super::furthest_distance_from_origin;

    #[test]
    fn examples() {
        assert_eq!(furthest_distance_from_origin("L_RL__R".into()), 3);
        assert_eq!(furthest_distance_from_origin("_R__LL_".into()), 5);
        assert_eq!(furthest_distance_from_origin("_______".into()), 7);
    }
}
