package leetcode

import "testing"

func TestAddTwoNumbersExampleOne(t *testing.T) {
	l1 := vecToList([]int{2, 4, 3})
	l2 := vecToList([]int{5, 6, 4})
	got := listToVec(addTwoNumbers(l1, l2))
	want := []int{7, 0, 8}
	if !sameInts(got, want) {
		t.Fatalf("addTwoNumbers() = %v, want %v", got, want)
	}
}

func TestAddTwoNumbersExampleTwo(t *testing.T) {
	l1 := vecToList([]int{0})
	l2 := vecToList([]int{0})
	got := listToVec(addTwoNumbers(l1, l2))
	want := []int{0}
	if !sameInts(got, want) {
		t.Fatalf("addTwoNumbers() = %v, want %v", got, want)
	}
}

func TestAddTwoNumbersExampleThree(t *testing.T) {
	l1 := vecToList([]int{9, 9, 9, 9, 9, 9, 9})
	l2 := vecToList([]int{9, 9, 9, 9})
	got := listToVec(addTwoNumbers(l1, l2))
	want := []int{8, 9, 9, 9, 0, 0, 0, 1}
	if !sameInts(got, want) {
		t.Fatalf("addTwoNumbers() = %v, want %v", got, want)
	}
}
