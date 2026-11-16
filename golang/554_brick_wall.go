package leetcode

// LeetCode #554 - Brick Wall
//
// The best line is the edge position that the most rows already share, so the fewest bricks are crossed.
func leastBricks(wall [][]int) int {
	edges := map[int]int{}
	for _, row := range wall {
		position := 0
		for _, brick := range row[:len(row)-1] {
			position += brick
			edges[position]++
		}
	}
	most := 0
	for _, count := range edges {
		most = max(most, count)
	}
	return len(wall) - most
}
