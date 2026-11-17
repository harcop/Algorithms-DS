package leetcode

import "testing"

func TestMaxVacationDaysExamples(t *testing.T) {
	flights := [][]int{{0, 1, 1}, {1, 0, 1}, {1, 1, 0}}
	days := [][]int{{1, 3, 1}, {6, 0, 3}, {3, 3, 3}}
	if got := maxVacationDays(flights, days); got != 12 {
		t.Fatalf("maxVacationDays = %d, want 12", got)
	}
	flights = [][]int{{0, 0, 0}, {0, 0, 0}, {0, 0, 0}}
	days = [][]int{{1, 1, 1}, {7, 7, 7}, {7, 7, 7}}
	if got := maxVacationDays(flights, days); got != 3 {
		t.Fatalf("maxVacationDays = %d, want 3", got)
	}
}
