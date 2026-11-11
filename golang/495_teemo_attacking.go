package leetcode

// LeetCode #495 - Teemo Attacking
//
// A new attack resets the poison, so overlapping intervals only last until the next attack.
func findPoisonedDuration(timeSeries []int, duration int) int {
	if len(timeSeries) == 0 {
		return 0
	}
	poisoned := 0
	for i := 1; i < len(timeSeries); i++ {
		poisoned += min(timeSeries[i]-timeSeries[i-1], duration)
	}
	return poisoned + duration
}
