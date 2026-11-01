package leetcode

import "sort"

// LeetCode #332 - Reconstruct Itinerary
//
// Hierholzer's walk pops the lexicographically largest unused ticket last, then reverses the trail.
func findItinerary(tickets [][]string) []string {
	graph := map[string][]string{}
	for _, ticket := range tickets {
		graph[ticket[0]] = append(graph[ticket[0]], ticket[1])
	}
	for from, dests := range graph {
		sort.Slice(dests, func(i, j int) bool { return dests[i] > dests[j] })
		graph[from] = dests
	}
	route := make([]string, 0, len(tickets)+1)
	var fly func(string)
	fly = func(airport string) {
		for len(graph[airport]) > 0 {
			next := graph[airport][len(graph[airport])-1]
			graph[airport] = graph[airport][:len(graph[airport])-1]
			fly(next)
		}
		route = append(route, airport)
	}
	fly("JFK")
	for i, j := 0, len(route)-1; i < j; i, j = i+1, j-1 {
		route[i], route[j] = route[j], route[i]
	}
	return route
}
