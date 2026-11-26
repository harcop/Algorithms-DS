package leetcode

// LeetCode #684 - Redundant Connection
//
// The extra edge is the one that joins two nodes already in the same component.
func findRedundantConnection(edges [][]int) []int {
	set := newConnSet(len(edges))
	var extra []int
	for _, edge := range edges {
		if !set.union(edge[0], edge[1]) {
			extra = []int{edge[0], edge[1]}
		}
	}
	return extra
}

type connSet struct {
	parent []int
}

func newConnSet(n int) *connSet {
	parent := make([]int, n+1)
	for i := range parent {
		parent[i] = i
	}
	return &connSet{parent: parent}
}

func (s *connSet) find(x int) int {
	if s.parent[x] != x {
		s.parent[x] = s.find(s.parent[x])
	}
	return s.parent[x]
}

func (s *connSet) union(a, b int) bool {
	ra, rb := s.find(a), s.find(b)
	if ra == rb {
		return false
	}
	s.parent[ra] = rb
	return true
}
