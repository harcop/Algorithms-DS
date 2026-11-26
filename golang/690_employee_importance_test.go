package leetcode

import "testing"

func TestGetImportanceExample(t *testing.T) {
	employees := []*Employee{
		{Id: 1, Importance: 5, Subordinates: []int{2, 3}},
		{Id: 2, Importance: 3},
		{Id: 3, Importance: 3},
	}
	if got := getImportance(employees, 1); got != 11 {
		t.Fatalf("getImportance(1) = %d, want 11", got)
	}
	if got := getImportance(employees, 2); got != 3 {
		t.Fatalf("getImportance(2) = %d, want 3", got)
	}
}
