package leetcode

import "testing"

func TestValidTreeExampleOne(t *testing.T) {
	edges := [][]int{{0, 1}, {0, 2}, {0, 3}, {1, 4}}
	if !validTree(5, edges) {
		t.Fatal("validTree() = false, want true")
	}
}

func TestValidTreeExampleTwo(t *testing.T) {
	edges := [][]int{{0, 1}, {1, 2}, {2, 3}, {1, 3}, {1, 4}}
	if validTree(5, edges) {
		t.Fatal("validTree() = true, want false")
	}
}
