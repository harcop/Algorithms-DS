package leetcode

// LeetCode #207 - Course Schedule
//
// A course can be taken when every prerequisite has already been taken; a leftover cycle means no.
func canFinish(numCourses int, prerequisites [][]int) bool {
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
	done := 0
	for head := 0; head < len(queue); head++ {
		course := queue[head]
		done++
		for _, later := range next[course] {
			indegree[later]--
			if indegree[later] == 0 {
				queue = append(queue, later)
			}
		}
	}
	return done == numCourses
}
