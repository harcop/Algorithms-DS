package leetcode

import "sort"

// LeetCode #587 - Erect the Fence
//
// A monotone chain keeps left turns, including trees that sit on a straight edge.
func outerTrees(trees [][]int) [][]int {
	n := len(trees)
	if n <= 1 {
		return trees
	}
	points := append([][]int(nil), trees...)
	sort.Slice(points, func(i, j int) bool {
		if points[i][0] == points[j][0] {
			return points[i][1] < points[j][1]
		}
		return points[i][0] < points[j][0]
	})
	cross := func(origin, a, b []int) int {
		return (a[0]-origin[0])*(b[1]-origin[1]) - (a[1]-origin[1])*(b[0]-origin[0])
	}
	hull := make([][]int, 0, n)
	for _, point := range points {
		for len(hull) >= 2 && cross(hull[len(hull)-2], hull[len(hull)-1], point) < 0 {
			hull = hull[:len(hull)-1]
		}
		hull = append(hull, point)
	}
	lower := len(hull)
	for i := n - 2; i >= 0; i-- {
		point := points[i]
		for len(hull) > lower && cross(hull[len(hull)-2], hull[len(hull)-1], point) < 0 {
			hull = hull[:len(hull)-1]
		}
		hull = append(hull, point)
	}
	hull = hull[:len(hull)-1]
	seen := map[[2]int]struct{}{}
	out := make([][]int, 0, len(hull))
	for _, point := range hull {
		key := [2]int{point[0], point[1]}
		if _, ok := seen[key]; ok {
			continue
		}
		seen[key] = struct{}{}
		out = append(out, point)
	}
	return out
}
