package leetcode

import (
	"strings"
	"testing"
)

func TestRisingTemperatureSQL(t *testing.T) {
	if !strings.Contains(strings.ToLower(risingTemperatureSQL), "weather") {
		t.Fatalf("risingTemperatureSQL = %q, want a weather join", risingTemperatureSQL)
	}
}
