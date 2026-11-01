package leetcode

import "testing"

func TestBulbSwitchExampleOne(t *testing.T) {
	if got := bulbSwitch(3); got != 1 {
		t.Fatalf("bulbSwitch() = %d, want 1", got)
	}
}
