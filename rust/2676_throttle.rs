/// LeetCode #2676 - Throttle (JS problem; simulated timer analogue)
/// Simulate throttle: given delay `t` and calls as (time, inputs), return fired events.
fn throttle_simulate(t: i32, calls: &[(i32, Vec<i32>)]) -> Vec<(i32, Vec<i32>)> {
    let mut out = Vec::new();
    let mut pending = false;
    let mut unlock_at = 0;
    let mut next_args: Option<Vec<i32>> = None;

    let mut i = 0;
    while i < calls.len() || next_args.is_some() {
        if pending && next_args.is_some() && (i >= calls.len() || unlock_at <= calls[i].0) {
            let now = unlock_at;
            let args = next_args.take().unwrap();
            out.push((now, args));
            unlock_at = now + t;
            pending = true;
            continue;
        }
        if i >= calls.len() {
            break;
        }
        let (ct, inputs) = &calls[i];
        let now = *ct;
        i += 1;
        if !pending || now >= unlock_at {
            out.push((now, inputs.clone()));
            pending = true;
            unlock_at = now + t;
            next_args = None;
        } else {
            next_args = Some(inputs.clone());
        }
    }
    out
}

fn main() {
    println!("{:?}", throttle_simulate(100, &[(20, vec![1])]));
}

#[cfg(test)]
mod tests {
    use super::throttle_simulate;

    #[test]
    fn example_one() {
        assert_eq!(
            throttle_simulate(100, &[(20, vec![1])]),
            vec![(20, vec![1])]
        );
    }

    #[test]
    fn example_two() {
        assert_eq!(
            throttle_simulate(50, &[(50, vec![1]), (75, vec![2])]),
            vec![(50, vec![1]), (100, vec![2])]
        );
    }

    #[test]
    fn example_three() {
        assert_eq!(
            throttle_simulate(
                70,
                &[
                    (50, vec![1]),
                    (75, vec![2]),
                    (90, vec![8]),
                    (140, vec![5, 7]),
                    (300, vec![9, 4]),
                ]
            ),
            vec![
                (50, vec![1]),
                (120, vec![8]),
                (190, vec![5, 7]),
                (300, vec![9, 4]),
            ]
        );
    }
}
