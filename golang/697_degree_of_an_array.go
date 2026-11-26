package leetcode

// LeetCode #697 - Degree of an Array
//
// The degree is the highest frequency, and the answer is the shortest span that still holds that frequency.
func findShortestSubArray(nums []int) int {
	type span struct {
		count int
		first int
		last  int
	}
	info := map[int]*span{}
	degree, best := 0, len(nums)
	for i, num := range nums {
		item := info[num]
		if item == nil {
			item = &span{first: i}
			info[num] = item
		}
		item.count++
		item.last = i
		length := item.last - item.first + 1
		if item.count > degree {
			degree = item.count
			best = length
		} else if item.count == degree {
			best = min(best, length)
		}
	}
	return best
}
