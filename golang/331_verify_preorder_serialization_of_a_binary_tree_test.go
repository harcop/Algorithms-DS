package leetcode

import "testing"

func TestIsValidSerializationExampleOne(t *testing.T) {
	if !isValidSerialization("9,3,4,#,#,1,#,#,2,#,6,#,#") {
		t.Fatal("isValidSerialization() = false, want true")
	}
}

func TestIsValidSerializationExampleTwo(t *testing.T) {
	if isValidSerialization("1,#") {
		t.Fatal("isValidSerialization() = true, want false")
	}
}
