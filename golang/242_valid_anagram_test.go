package leetcode

import "testing"

func TestIsAnagramExampleOne(t *testing.T) {
	if !isAnagram("anagram", "nagaram") {
		t.Fatal("isAnagram() = false, want true")
	}
}

func TestIsAnagramExampleTwo(t *testing.T) {
	if isAnagram("rat", "car") {
		t.Fatal("isAnagram() = true, want false")
	}
}
