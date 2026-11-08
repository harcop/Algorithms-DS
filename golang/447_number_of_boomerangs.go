package leetcode

// LeetCode #447 - Number of Boomerangs
//
// For each point, every pair of neighbors at the same distance can be ordered in two ways.
func numberOfBoomerangs(points [][]int) int {
	total := 0
	for i := range points {
		distances := map[int]int{}
		for j := range points {
			if i == j {
				continue
			}
			dx := points[i][0] - points[j][0]
			dy := points[i][1] - points[j][1]
			distances[dx*dx+dy*dy]++
		}
		for _, count := range distances {
			total += count * (count - 1)
		}
	}
	return total
}
