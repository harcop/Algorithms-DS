package leetcode

// LeetCode #323 - Number of Connected Components in an Undirected Graph
//
// Union the endpoints of each edge and count how many roots remain.
func countComponents(n int, edges [][]int) int {
	parent := make([]int, n)
	for i := range parent {
		parent[i] = i
	}
	var find func(int) int
	find = func(x int) int {
		for parent[x] != x {
			parent[x] = parent[parent[x]]
			x = parent[x]
		}
		return x
	}
	components := n
	for _, edge := range edges {
		ra, rb := find(edge[0]), find(edge[1])
		if ra == rb {
			continue
		}
		parent[ra] = rb
		components--
	}
	return components
}
