package leetcode

// LeetCode #546 - Remove Boxes
//
// Removing a run scores its square, and attaching more boxes of the same color later can make a bigger run.
func removeBoxes(boxes []int) int {
	n := len(boxes)
	if n == 0 {
		return 0
	}
	memo := make([][][]int, n)
	for i := range memo {
		memo[i] = make([][]int, n)
		for j := range memo[i] {
			memo[i][j] = make([]int, n)
			for k := range memo[i][j] {
				memo[i][j][k] = -1
			}
		}
	}
	var score func(left, right, extra int) int
	score = func(left, right, extra int) int {
		if left > right {
			return 0
		}
		if memo[left][right][extra] != -1 {
			return memo[left][right][extra]
		}
		end, attached := right, extra
		for end > left && boxes[end-1] == boxes[end] {
			end--
			attached++
		}
		best := score(left, end-1, 0) + (attached+1)*(attached+1)
		for i := left; i < end; i++ {
			if boxes[i] != boxes[end] || (i > left && boxes[i] == boxes[i-1]) {
				continue
			}
			best = max(best, score(left, i, attached+1)+score(i+1, end-1, 0))
		}
		memo[left][right][extra] = best
		return best
	}
	return score(0, n-1, 0)
}
