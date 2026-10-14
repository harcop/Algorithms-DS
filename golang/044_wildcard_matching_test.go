package leetcode

import "testing"

func TestIsWildcardMatchExampleOne(t *testing.T) {
	if isWildcardMatch("aa", "a") {
		t.Fatal("isWildcardMatch() = true, want false")
	}
}

func TestIsWildcardMatchExampleTwo(t *testing.T) {
	if !isWildcardMatch("aa", "*") {
		t.Fatal("isWildcardMatch() = false, want true")
	}
}

func TestIsWildcardMatchExampleThree(t *testing.T) {
	if isWildcardMatch("cb", "?a") {
		t.Fatal("isWildcardMatch() = true, want false")
	}
}
