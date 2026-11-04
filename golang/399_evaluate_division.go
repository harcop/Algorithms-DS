package leetcode

// LeetCode #399 - Evaluate Division
//
// Each equation is an edge, and a query is the product of weights along a path.
func calcEquation(equations [][]string, values []float64, queries [][]string) []float64 {
	graph := map[string]map[string]float64{}
	for i, equation := range equations {
		a, b := equation[0], equation[1]
		if graph[a] == nil {
			graph[a] = map[string]float64{}
		}
		if graph[b] == nil {
			graph[b] = map[string]float64{}
		}
		graph[a][b] = values[i]
		graph[b][a] = 1 / values[i]
	}
	out := make([]float64, len(queries))
	for i, query := range queries {
		out[i] = -1
		if graph[query[0]] == nil || graph[query[1]] == nil {
			continue
		}
		if value, ok := divideQuery(graph, query[0], query[1], map[string]bool{}); ok {
			out[i] = value
		}
	}
	return out
}

func divideQuery(graph map[string]map[string]float64, cur, target string, seen map[string]bool) (float64, bool) {
	if cur == target {
		return 1, true
	}
	seen[cur] = true
	for next, weight := range graph[cur] {
		if seen[next] {
			continue
		}
		if value, ok := divideQuery(graph, next, target, seen); ok {
			return weight * value, true
		}
	}
	return 0, false
}
