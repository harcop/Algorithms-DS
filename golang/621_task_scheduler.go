package leetcode

// LeetCode #621 - Task Scheduler
//
// The most frequent task sets the frame, and other tasks fill the gaps it leaves.
func leastInterval(tasks []byte, n int) int {
	counts := make([]int, 26)
	most := 0
	for _, task := range tasks {
		counts[task-'A']++
		if counts[task-'A'] > most {
			most = counts[task-'A']
		}
	}
	tied := 0
	for _, count := range counts {
		if count == most {
			tied++
		}
	}
	slots := (most-1)*(n+1) + tied
	if slots < len(tasks) {
		return len(tasks)
	}
	return slots
}
