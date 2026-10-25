package leetcode

// LeetCode #261 - Graph Valid Tree
//
// A tree on n nodes has exactly n-1 edges and no cycle.
func validTree(n int, edges [][]int) bool {
	if len(edges) != n-1 {
		return false
	}
	parent := make([]int, n)
	for i := range parent {
		parent[i] = i
	}
	var find func(int) int
	find = func(x int) int {
		if parent[x] != x {
			parent[x] = find(parent[x])
		}
		return parent[x]
	}
	for _, edge := range edges {
		a, b := find(edge[0]), find(edge[1])
		if a == b {
			return false
		}
		parent[b] = a
	}
	root := find(0)
	for i := 1; i < n; i++ {
		if find(i) != root {
			return false
		}
	}
	return true
}
