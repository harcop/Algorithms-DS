package leetcode

// LeetCode #433 - Minimum Genetic Mutation
//
// Breadth-first search tries each one-letter change that the bank still allows.
func minMutation(startGene, endGene string, bank []string) int {
	if startGene == endGene {
		return 0
	}
	allowed := make(map[string]bool, len(bank))
	for _, gene := range bank {
		allowed[gene] = true
	}
	if !allowed[endGene] {
		return -1
	}
	type geneStep struct {
		gene string
		dist int
	}
	queue := []geneStep{{gene: startGene, dist: 0}}
	seen := map[string]bool{startGene: true}
	bases := []byte{'A', 'C', 'G', 'T'}
	for len(queue) > 0 {
		cur := queue[0]
		queue = queue[1:]
		gene := []byte(cur.gene)
		for i := range gene {
			original := gene[i]
			for _, base := range bases {
				if base == original {
					continue
				}
				gene[i] = base
				next := string(gene)
				if next == endGene {
					return cur.dist + 1
				}
				if allowed[next] && !seen[next] {
					seen[next] = true
					queue = append(queue, geneStep{gene: next, dist: cur.dist + 1})
				}
			}
			gene[i] = original
		}
	}
	return -1
}
