package leetcode

// LeetCode #364 - Nested List Weight Sum II
//
// An integer's weight is one more than the distance from the deepest level.
func depthSumInverse(nested []NestedInteger) int {
	var values [][2]int
	var walk func([]NestedInteger, int)
	walk = func(items []NestedInteger, depth int) {
		for _, item := range items {
			if item.value != nil {
				values = append(values, [2]int{*item.value, depth})
				continue
			}
			walk(item.list, depth+1)
		}
	}
	walk(nested, 1)
	maxDepth := 1
	for _, value := range values {
		if value[1] > maxDepth {
			maxDepth = value[1]
		}
	}
	sum := 0
	for _, value := range values {
		sum += value[0] * (maxDepth - value[1] + 1)
	}
	return sum
}
