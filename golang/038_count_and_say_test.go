package leetcode

import "testing"

func TestCountAndSayExampleOne(t *testing.T) {
	got := countAndSay(1)
	if got != "1" {
		t.Fatalf("countAndSay() = %q, want %q", got, "1")
	}
}

func TestCountAndSayExampleTwo(t *testing.T) {
	got := countAndSay(4)
	if got != "1211" {
		t.Fatalf("countAndSay() = %q, want %q", got, "1211")
	}
}
