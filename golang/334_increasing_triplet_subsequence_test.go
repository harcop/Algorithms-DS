package leetcode

import "testing"

func TestIncreasingTripletExampleOne(t *testing.T) {
	if !increasingTriplet([]int{1, 2, 3, 4, 5}) {
		t.Fatal("increasingTriplet() = false, want true")
	}
}

func TestIncreasingTripletExampleTwo(t *testing.T) {
	if increasingTriplet([]int{5, 4, 3, 2, 1}) {
		t.Fatal("increasingTriplet() = true, want false")
	}
}
