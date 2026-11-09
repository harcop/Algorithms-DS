package leetcode

// LeetCode #469 - Convex Polygon
//
// Consecutive edges of a convex polygon all turn the same way, so their cross products keep one sign.
func isConvex(points [][]int) bool {
	n := len(points)
	previous := int64(0)
	for i := 0; i < n; i++ {
		ax, ay := int64(points[i][0]), int64(points[i][1])
		bx, by := int64(points[(i+1)%n][0]), int64(points[(i+1)%n][1])
		cx, cy := int64(points[(i+2)%n][0]), int64(points[(i+2)%n][1])
		cross := (bx-ax)*(cy-by) - (by-ay)*(cx-bx)
		if cross == 0 {
			continue
		}
		if cross*previous < 0 {
			return false
		}
		previous = cross
	}
	return true
}
