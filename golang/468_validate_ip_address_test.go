package leetcode

import "testing"

func TestValidIPAddressExamples(t *testing.T) {
	if got := validIPAddress("172.16.254.1"); got != "IPv4" {
		t.Fatalf("validIPAddress() = %q, want IPv4", got)
	}
	if got := validIPAddress("2001:0db8:85a3:0:0:8A2E:0370:7334"); got != "IPv6" {
		t.Fatalf("validIPAddress() = %q, want IPv6", got)
	}
	if got := validIPAddress("256.256.256.256"); got != "Neither" {
		t.Fatalf("validIPAddress() = %q, want Neither", got)
	}
}
