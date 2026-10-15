package leetcode

import "testing"

func TestAddBinaryExampleOne(t *testing.T) {
	got := addBinary("11", "1")
	if got != "100" {
		t.Fatalf("addBinary() = %q, want %q", got, "100")
	}
}

func TestAddBinaryExampleTwo(t *testing.T) {
	got := addBinary("1010", "1011")
	if got != "10101" {
		t.Fatalf("addBinary() = %q, want %q", got, "10101")
	}
}
