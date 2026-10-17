package leetcode

import "testing"

func TestGetRowExampleOne(t *testing.T) {
	got := getRow(3)
	want := []int{1, 3, 3, 1}
	if !sameInts(got, want) {
		t.Fatalf("getRow() = %v, want %v", got, want)
	}
}

func TestGetRowExampleTwo(t *testing.T) {
	got := getRow(0)
	want := []int{1}
	if !sameInts(got, want) {
		t.Fatalf("getRow() = %v, want %v", got, want)
	}
}

func TestGetRowExampleThree(t *testing.T) {
	got := getRow(1)
	want := []int{1, 1}
	if !sameInts(got, want) {
		t.Fatalf("getRow() = %v, want %v", got, want)
	}
}
