package leetcode

import "sort"

// LeetCode #310 - Minimum Height Trees
//
// Peel the leaves until one or two central nodes remain.
func findMinHeightTrees(n int, edges [][]int) []int {
	if n == 1 {
		return []int{0}
	}
	next := make([][]int, n)
	degree := make([]int, n)
	for _, edge := range edges {
		u, v := edge[0], edge[1]
		next[u] = append(next[u], v)
		next[v] = append(next[v], u)
		degree[u]++
		degree[v]++
	}
	queue := make([]int, 0)
	for node, deg := range degree {
		if deg == 1 {
			queue = append(queue, node)
		}
	}
	remaining := n
	for remaining > 2 {
		layer := len(queue)
		for i := 0; i < layer; i++ {
			node := queue[0]
			queue = queue[1:]
			remaining--
			for _, neighbor := range next[node] {
				degree[neighbor]--
				if degree[neighbor] == 1 {
					queue = append(queue, neighbor)
				}
			}
		}
	}
	sort.Ints(queue)
	return queue
}
