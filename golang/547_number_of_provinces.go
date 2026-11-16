package leetcode

// LeetCode #547 - Number of Provinces
//
// Each unvisited city starts a province, and a search marks every city connected to it.
func findCircleNum(isConnected [][]int) int {
	n := len(isConnected)
	seen := make([]bool, n)
	provinces := 0
	var visit func(city int)
	visit = func(city int) {
		seen[city] = true
		for next := 0; next < n; next++ {
			if isConnected[city][next] == 1 && !seen[next] {
				visit(next)
			}
		}
	}
	for city := 0; city < n; city++ {
		if seen[city] {
			continue
		}
		provinces++
		visit(city)
	}
	return provinces
}
