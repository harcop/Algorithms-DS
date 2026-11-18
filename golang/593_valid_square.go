package leetcode

import "sort"

// LeetCode #593 - Valid Square
//
// Four equal positive sides and two equal diagonals are a square, in any point order.
func validSquare(p1 []int, p2 []int, p3 []int, p4 []int) bool {
	distances := []int{
		squaredDistance(p1, p2), squaredDistance(p1, p3), squaredDistance(p1, p4),
		squaredDistance(p2, p3), squaredDistance(p2, p4), squaredDistance(p3, p4),
	}
	sort.Ints(distances)
	side := distances[0]
	return side > 0 && distances[1] == side && distances[2] == side && distances[3] == side &&
		distances[4] == side*2 && distances[5] == side*2
}

func squaredDistance(a, b []int) int {
	dx, dy := a[0]-b[0], a[1]-b[1]
	return dx*dx + dy*dy
}
