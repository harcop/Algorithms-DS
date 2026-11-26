package leetcode

// LeetCode #685 - Redundant Connection II
//
// A node with two parents, a cycle, or both, identifies the directed edge that has to go.
func findRedundantDirectedConnection(edges [][]int) []int {
	n := len(edges)
	parent := make([]int, n+1)
	var first, second []int
	skip := -1
	for i, edge := range edges {
		from, to := edge[0], edge[1]
		if parent[to] == 0 {
			parent[to] = from
			continue
		}
		first = []int{parent[to], to}
		second = []int{from, to}
		skip = i
	}
	set := newConnSet(n)
	for i, edge := range edges {
		if i == skip {
			continue
		}
		if !set.union(edge[0], edge[1]) {
			if first == nil {
				return []int{edge[0], edge[1]}
			}
			return first
		}
	}
	return second
}
