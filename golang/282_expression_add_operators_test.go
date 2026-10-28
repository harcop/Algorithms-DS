package leetcode

import "testing"

func TestAddOperatorsExampleOne(t *testing.T) {
	got := addOperators("123", 6)
	want := []string{"1*2*3", "1+2+3"}
	if !sameStrings(got, want) {
		t.Fatalf("addOperators() = %v, want %v", got, want)
	}
}
