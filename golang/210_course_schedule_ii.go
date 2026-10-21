package leetcode

// LeetCode #210 - Course Schedule II
//
// Take courses with no remaining prerequisites; an unfinished graph has a cycle.
func findOrder(numCourses int, prerequisites [][]int) []int {
	next := make([][]int, numCourses)
	indegree := make([]int, numCourses)
	for _, edge := range prerequisites {
		course, pre := edge[0], edge[1]
		next[pre] = append(next[pre], course)
		indegree[course]++
	}
	queue := make([]int, 0, numCourses)
	for course, degree := range indegree {
		if degree == 0 {
			queue = append(queue, course)
		}
	}
	order := make([]int, 0, numCourses)
	for head := 0; head < len(queue); head++ {
		course := queue[head]
		order = append(order, course)
		for _, later := range next[course] {
			indegree[later]--
			if indegree[later] == 0 {
				queue = append(queue, later)
			}
		}
	}
	if len(order) != numCourses {
		return []int{}
	}
	return order
}
