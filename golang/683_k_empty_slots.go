package leetcode

// LeetCode #683 - K Empty Slots
//
// A window of k empty slots is valid once both ends have bloomed and every flower between them blooms later.
func kEmptySlots(bulbs []int, k int) int {
	days := make([]int, len(bulbs))
	for day, position := range bulbs {
		days[position-1] = day + 1
	}
	answer := len(bulbs) + 1
	left, right := 0, k+1
	for right < len(days) {
		valid := true
		for i := left + 1; i < right; i++ {
			if days[i] < days[left] || days[i] < days[right] {
				left, right = i, i+k+1
				valid = false
				break
			}
		}
		if !valid {
			continue
		}
		answer = min(answer, max(days[left], days[right]))
		left = right
		right = left + k + 1
	}
	if answer > len(bulbs) {
		return -1
	}
	return answer
}
