package leetcode

// LeetCode #356 - Line Reflection
//
// A vertical mirror sits halfway between the leftmost and rightmost x, and every point needs its pair.
func isReflected(points [][]int) bool {
	if len(points) == 0 {
		return true
	}
	minX, maxX := points[0][0], points[0][0]
	seen := make(map[[2]int]struct{}, len(points))
	for _, point := range points {
		if point[0] < minX {
			minX = point[0]
		}
		if point[0] > maxX {
			maxX = point[0]
		}
		seen[[2]int{point[0], point[1]}] = struct{}{}
	}
	pivot := int64(minX) + int64(maxX)
	for _, point := range points {
		mirror := int(pivot - int64(point[0]))
		if _, ok := seen[[2]int{mirror, point[1]}]; !ok {
			return false
		}
	}
	return true
}
