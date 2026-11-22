package leetcode

import "testing"

func TestScheduleCourseExamples(t *testing.T) {
	courses := [][]int{{100, 200}, {200, 1300}, {1000, 1250}, {2000, 3200}}
	if got := scheduleCourse(courses); got != 3 {
		t.Fatalf("scheduleCourse() = %d, want 3", got)
	}
	if got := scheduleCourse([][]int{{1, 2}}); got != 1 {
		t.Fatalf("scheduleCourse([[1,2]]) = %d, want 1", got)
	}
	if got := scheduleCourse([][]int{{3, 2}, {4, 3}}); got != 0 {
		t.Fatalf("scheduleCourse() = %d, want 0", got)
	}
}
