/// LeetCode #630 - Course Schedule III
fn schedule_course(courses: Vec<Vec<i32>>) -> i32 {
    let mut courses = courses;
    courses.sort_by_key(|c| c[1]);
    let mut heap: Vec<i32> = vec![];
    let mut time = 0;
    fn push(heap: &mut Vec<i32>, x: i32) {
        heap.push(x);
        let mut i = heap.len() - 1;
        while i > 0 {
            let p = (i - 1) / 2;
            if heap[p] >= heap[i] {
                break;
            }
            heap.swap(p, i);
            i = p;
        }
    }
    fn pop(heap: &mut Vec<i32>) -> i32 {
        let n = heap.len();
        heap.swap(0, n - 1);
        let val = heap.pop().unwrap();
        let mut i = 0;
        loop {
            let l = i * 2 + 1;
            let r = i * 2 + 2;
            let mut largest = i;
            if l < heap.len() && heap[l] > heap[largest] {
                largest = l;
            }
            if r < heap.len() && heap[r] > heap[largest] {
                largest = r;
            }
            if largest == i {
                break;
            }
            heap.swap(i, largest);
            i = largest;
        }
        val
    }
    for c in courses {
        let (d, last) = (c[0], c[1]);
        if time + d <= last {
            time += d;
            push(&mut heap, d);
        } else if let Some(&max_d) = heap.first() {
            if max_d > d {
                time += d - pop(&mut heap);
                push(&mut heap, d);
            }
        }
    }
    heap.len() as i32
}

fn main() {
    let courses = vec![
        vec![100, 200],
        vec![200, 1300],
        vec![1000, 1250],
        vec![2000, 3200],
    ];
    println!("{}", schedule_course(courses));
}

#[cfg(test)]
mod tests {
    use super::schedule_course;

    #[test]
    fn example_one() {
        let courses = vec![
            vec![100, 200],
            vec![200, 1300],
            vec![1000, 1250],
            vec![2000, 3200],
        ];
        assert_eq!(schedule_course(courses), 3);
    }

    #[test]
    fn example_two() {
        assert_eq!(schedule_course(vec![vec![1, 2]]), 1);
    }

    #[test]
    fn example_three() {
        assert_eq!(schedule_course(vec![vec![3, 2], vec![4, 3]]), 0);
    }
}
