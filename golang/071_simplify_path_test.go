package leetcode

import "testing"

func TestSimplifyPathExampleOne(t *testing.T) {
	got := simplifyPath("/home/")
	if got != "/home" {
		t.Fatalf("simplifyPath() = %q, want %q", got, "/home")
	}
}

func TestSimplifyPathExampleTwo(t *testing.T) {
	got := simplifyPath("/home//foo/")
	if got != "/home/foo" {
		t.Fatalf("simplifyPath() = %q, want %q", got, "/home/foo")
	}
}

func TestSimplifyPathExampleThree(t *testing.T) {
	got := simplifyPath("/home/user/Documents/../Pictures")
	if got != "/home/user/Pictures" {
		t.Fatalf("simplifyPath() = %q, want %q", got, "/home/user/Pictures")
	}
}
