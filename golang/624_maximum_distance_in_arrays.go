package leetcode

// LeetCode #624 - Maximum Distance in Arrays
//
// Each array contributes its ends, and the widest gap uses an end from a different array.
func maxDistance(arrays [][]int) int {
	best := 0
	low := arrays[0][0]
	high := arrays[0][len(arrays[0])-1]
	for i := 1; i < len(arrays); i++ {
		row := arrays[i]
		first, last := row[0], row[len(row)-1]
		if last-low > best {
			best = last - low
		}
		if high-first > best {
			best = high - first
		}
		if first < low {
			low = first
		}
		if last > high {
			high = last
		}
	}
	return best
}
