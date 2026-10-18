package leetcode

// GraphNode is an undirected graph node.
type GraphNode struct {
	Val       int
	Neighbors []*GraphNode
}

// LeetCode #133 - Clone Graph
//
// Copy each node once, then attach copies of its neighbors.
func cloneGraph(node *GraphNode) *GraphNode {
	if node == nil {
		return nil
	}
	clones := map[*GraphNode]*GraphNode{}

	var dfs func(*GraphNode) *GraphNode
	dfs = func(current *GraphNode) *GraphNode {
		if copy, ok := clones[current]; ok {
			return copy
		}
		copy := &GraphNode{Val: current.Val}
		clones[current] = copy
		for _, neighbor := range current.Neighbors {
			copy.Neighbors = append(copy.Neighbors, dfs(neighbor))
		}
		return copy
	}
	return dfs(node)
}
