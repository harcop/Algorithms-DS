package leetcode

// LeetCode #466 - Count The Repetitions
//
// Walk s1 copies against s2, and once a match position repeats, jump over the whole cycle.
func getMaxRepetitions(s1 string, n1 int, s2 string, n2 int) int {
	if len(s2) == 0 || n1 == 0 || n2 == 0 {
		return 0
	}
	type progress struct {
		repeat int
		count  int64
	}
	seen := map[int]progress{}
	index := 0
	count := int64(0)
	repeat := 0
	for repeat < n1 {
		if prev, ok := seen[index]; ok {
			deltaRepeat := repeat - prev.repeat
			deltaCount := count - prev.count
			remaining := n1 - repeat
			if deltaRepeat > 0 && remaining >= deltaRepeat {
				times := remaining / deltaRepeat
				count += int64(times) * deltaCount
				repeat += times * deltaRepeat
				seen = map[int]progress{}
				continue
			}
		}
		seen[index] = progress{repeat, count}
		for i := 0; i < len(s1); i++ {
			if s1[i] != s2[index] {
				continue
			}
			index++
			if index == len(s2) {
				count++
				index = 0
			}
		}
		repeat++
	}
	return int(count / int64(n2))
}
