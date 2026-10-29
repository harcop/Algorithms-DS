package leetcode

import "testing"

func TestGetHintExampleOne(t *testing.T) {
	if got := getHint("1807", "7810"); got != "1A3B" {
		t.Fatalf("getHint() = %q, want %q", got, "1A3B")
	}
}

func TestGetHintExampleTwo(t *testing.T) {
	if got := getHint("1123", "0111"); got != "1A1B" {
		t.Fatalf("getHint() = %q, want %q", got, "1A1B")
	}
}
