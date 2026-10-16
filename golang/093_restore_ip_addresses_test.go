package leetcode

import "testing"

func TestRestoreIpAddressesExampleOne(t *testing.T) {
	got := restoreIpAddresses("25525511135")
	want := []string{"255.255.11.135", "255.255.111.35"}
	if !sameStrings(got, want) {
		t.Fatalf("restoreIpAddresses() = %v, want %v", got, want)
	}
}

func TestRestoreIpAddressesExampleTwo(t *testing.T) {
	got := restoreIpAddresses("0000")
	want := []string{"0.0.0.0"}
	if !sameStrings(got, want) {
		t.Fatalf("restoreIpAddresses() = %v, want %v", got, want)
	}
}
