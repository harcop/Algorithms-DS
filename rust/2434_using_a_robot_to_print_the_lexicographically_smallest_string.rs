/// LeetCode #2434 - Using a Robot to Print the Lexicographically Smallest String
fn robot_with_string(s: String) -> String {
    let mut remaining = [0; 26];
    for byte in s.bytes() {
        remaining[(byte - b'a') as usize] += 1;
    }

    let mut smallest = 0;
    let mut stack = Vec::with_capacity(s.len());
    let mut answer = Vec::with_capacity(s.len());

    for byte in s.bytes() {
        let index = (byte - b'a') as usize;
        remaining[index] -= 1;
        while smallest < 26 && remaining[smallest] == 0 {
            smallest += 1;
        }

        stack.push(byte);
        while let Some(&top) = stack.last() {
            if smallest == 26 || (top - b'a') as usize <= smallest {
                answer.push(stack.pop().unwrap());
            } else {
                break;
            }
        }
    }

    String::from_utf8(answer).unwrap()
}

fn main() {
    println!("{}", robot_with_string("zza".to_string()));
}

#[cfg(test)]
mod tests {
    use super::robot_with_string;

    #[test]
    fn example_one() {
        assert_eq!(robot_with_string("zza".to_string()), "azz");
    }

    #[test]
    fn example_two() {
        assert_eq!(robot_with_string("bac".to_string()), "abc");
    }

    #[test]
    fn example_three() {
        assert_eq!(robot_with_string("bdda".to_string()), "addb");
    }
}
