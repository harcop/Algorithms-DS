package leetcode

import "testing"

func TestIsStrobogrammaticExampleOne(t *testing.T) {
	if !isStrobogrammatic("69") {
		t.Fatal("isStrobogrammatic(69) = false, want true")
	}
}

func TestIsStrobogrammaticExampleTwo(t *testing.T) {
	if !isStrobogrammatic("88") {
		t.Fatal("isStrobogrammatic(88) = false, want true")
	}
}

func TestIsStrobogrammaticExampleThree(t *testing.T) {
	if isStrobogrammatic("962") {
		t.Fatal("isStrobogrammatic(962) = true, want false")
	}
}
