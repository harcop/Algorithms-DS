package leetcode

// LeetCode #444 - Sequence Reconstruction
//
// The original order is the only topological order when each step has exactly one node with indegree zero.
func sequenceReconstruction(org []int, seqs [][]int) bool {
	nodes := map[int]bool{}
	for _, seq := range seqs {
		for _, value := range seq {
			nodes[value] = true
		}
	}
	if len(org) == 0 {
		return len(seqs) == 0 || len(nodes) == 0
	}
	if len(nodes) != len(org) {
		return false
	}
	for _, value := range org {
		if !nodes[value] {
			return false
		}
	}

	graph := map[int][]int{}
	indegree := map[int]int{}
	for value := range nodes {
		indegree[value] = 0
	}
	seen := map[[2]int]bool{}
	for _, seq := range seqs {
		for i := 1; i < len(seq); i++ {
			edge := [2]int{seq[i-1], seq[i]}
			if seen[edge] {
				continue
			}
			seen[edge] = true
			graph[edge[0]] = append(graph[edge[0]], edge[1])
			indegree[edge[1]]++
		}
	}

	for _, need := range org {
		ready := 0
		only := 0
		for value, degree := range indegree {
			if degree == 0 {
				ready++
				only = value
			}
		}
		if ready != 1 || only != need {
			return false
		}
		delete(indegree, need)
		for _, next := range graph[need] {
			indegree[next]--
		}
	}
	return len(indegree) == 0
}
