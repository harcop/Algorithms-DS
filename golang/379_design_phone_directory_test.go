package leetcode

import "testing"

func TestPhoneDirectoryExample(t *testing.T) {
	directory := NewPhoneDirectory(3)
	if got := directory.Get(); got != 0 {
		t.Fatalf("Get() = %d, want 0", got)
	}
	if directory.Check(0) {
		t.Fatal("Check(0) = true, want false")
	}
	directory.Release(0)
	if !directory.Check(0) {
		t.Fatal("Check(0) = false, want true")
	}
	if got := directory.Get(); got != 0 {
		t.Fatalf("Get() = %d, want 0", got)
	}
	if !directory.Check(2) {
		t.Fatal("Check(2) = false, want true")
	}
	if got := directory.Get(); got != 1 {
		t.Fatalf("Get() = %d, want 1", got)
	}
	if got := directory.Get(); got != 2 {
		t.Fatalf("Get() = %d, want 2", got)
	}
	if got := directory.Get(); got != -1 {
		t.Fatalf("Get() = %d, want -1", got)
	}
	directory.Release(2)
	if got := directory.Get(); got != 2 {
		t.Fatalf("Get() = %d, want 2", got)
	}
}
