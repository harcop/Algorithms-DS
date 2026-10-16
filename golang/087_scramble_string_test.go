package leetcode

import "testing"

func TestIsScrambleExampleOne(t *testing.T) {
	if !isScramble("great", "rgeat") {
		t.Fatal("isScramble() = false, want true")
	}
}

func TestIsScrambleExampleTwo(t *testing.T) {
	if isScramble("abcde", "caebd") {
		t.Fatal("isScramble() = true, want false")
	}
}

func TestIsScrambleExampleThree(t *testing.T) {
	if !isScramble("a", "a") {
		t.Fatal("isScramble() = false, want true")
	}
}
