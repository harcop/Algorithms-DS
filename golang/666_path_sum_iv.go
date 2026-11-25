package leetcode

// LeetCode #666 - Path Sum IV
//
// Depth and position encode each node, so a walk from the root can add every root-to-leaf path.
func pathSumIV(nums []int) int {
	values := map[int]int{}
	for _, num := range nums {
		values[num/10] = num % 10
	}
	sum := 0
	var walk func(node, total int)
	walk = func(node, total int) {
		value, ok := values[node]
		if !ok {
			return
		}
		total += value
		depth, pos := node/10, node%10
		left := (depth+1)*10 + pos*2 - 1
		right := left + 1
		_, hasLeft := values[left]
		_, hasRight := values[right]
		if !hasLeft && !hasRight {
			sum += total
			return
		}
		walk(left, total)
		walk(right, total)
	}
	walk(11, 0)
	return sum
}
