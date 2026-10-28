package leetcode

// LeetCode #277 - Find the Celebrity
//
// A candidate who knows someone else is eliminated; then check that everyone else knows the survivor.
type Celebrity struct {
	n     int
	celeb int
}

func NewCelebrity(n int, celeb int) Celebrity {
	return Celebrity{n: n, celeb: celeb}
}

func (graph Celebrity) knows(a, b int) bool {
	return b == graph.celeb && a != graph.celeb
}

func (graph Celebrity) FindCelebrity() int {
	candidate := 0
	for person := 1; person < graph.n; person++ {
		if graph.knows(candidate, person) {
			candidate = person
		}
	}
	for person := 0; person < graph.n; person++ {
		if person != candidate && (graph.knows(candidate, person) || !graph.knows(person, candidate)) {
			return -1
		}
	}
	return candidate
}
