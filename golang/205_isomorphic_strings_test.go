package leetcode

import "testing"

func TestIsIsomorphicExampleOne(t *testing.T) {
	if !isIsomorphic("egg", "add") {
		t.Fatal("isIsomorphic() = false, want true")
	}
}

func TestIsIsomorphicExampleTwo(t *testing.T) {
	if isIsomorphic("foo", "bar") {
		t.Fatal("isIsomorphic() = true, want false")
	}
}

func TestIsIsomorphicExampleThree(t *testing.T) {
	if !isIsomorphic("paper", "title") {
		t.Fatal("isIsomorphic() = false, want true")
	}
}
