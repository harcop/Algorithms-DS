package leetcode

import (
	"strings"
	"testing"
)

func TestValidPhoneNumbersScript(t *testing.T) {
	if !strings.Contains(validPhoneNumbersScript, "grep") {
		t.Fatalf("validPhoneNumbersScript = %q, want grep", validPhoneNumbersScript)
	}
}
