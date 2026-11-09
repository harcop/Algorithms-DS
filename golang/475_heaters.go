package leetcode

import (
	"math"
	"sort"
)

// LeetCode #475 - Heaters
//
// Each house uses the nearer heater, and the radius is the largest of those distances.
func findRadius(houses []int, heaters []int) int {
	sort.Ints(houses)
	sort.Ints(heaters)
	radius := 0
	for _, house := range houses {
		pos := sort.Search(len(heaters), func(i int) bool { return heaters[i] >= house })
		dist := math.MaxInt
		if pos < len(heaters) {
			dist = absInt(heaters[pos] - house)
		}
		if pos > 0 {
			dist = min(dist, absInt(heaters[pos-1]-house))
		}
		radius = max(radius, dist)
	}
	return radius
}
