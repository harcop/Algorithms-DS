package leetcode

import "testing"

func TestLoggerExample(t *testing.T) {
	logger := NewLogger()
	checks := []struct {
		timestamp int
		message   string
		want      bool
	}{
		{1, "foo", true},
		{2, "bar", true},
		{3, "foo", false},
		{8, "bar", false},
		{10, "foo", false},
		{11, "foo", true},
	}
	for _, check := range checks {
		got := logger.ShouldPrintMessage(check.timestamp, check.message)
		if got != check.want {
			t.Fatalf("ShouldPrintMessage(%d, %q) = %v, want %v", check.timestamp, check.message, got, check.want)
		}
	}
}
