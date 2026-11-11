package leetcode

import "testing"

func TestLicenseKeyFormattingExamples(t *testing.T) {
	if got := licenseKeyFormatting("5F3Z-2e-9-w", 4); got != "5F3Z-2E9W" {
		t.Fatalf("licenseKeyFormatting() = %q, want 5F3Z-2E9W", got)
	}
	if got := licenseKeyFormatting("2-5g-3-J", 2); got != "2-5G-3J" {
		t.Fatalf("licenseKeyFormatting() = %q, want 2-5G-3J", got)
	}
}
