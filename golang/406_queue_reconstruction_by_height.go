package leetcode

import "sort"

// LeetCode #406 - Queue Reconstruction by Height
//
// Place taller people first, then insert each person at the index given by how many taller people stand in front.
func reconstructQueue(people [][]int) [][]int {
	sort.Slice(people, func(i, j int) bool {
		if people[i][0] != people[j][0] {
			return people[i][0] > people[j][0]
		}
		return people[i][1] < people[j][1]
	})
	out := make([][]int, 0, len(people))
	for _, person := range people {
		index := person[1]
		out = append(out, nil)
		copy(out[index+1:], out[index:])
		out[index] = person
	}
	return out
}
