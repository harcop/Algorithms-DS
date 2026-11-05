package leetcode

import "testing"

func TestReadBinaryWatchExample(t *testing.T) {
	got := readBinaryWatch(1)
	want := []string{"0:01", "0:02", "0:04", "0:08", "0:16", "0:32", "1:00", "2:00", "4:00", "8:00"}
	if len(got) != len(want) {
		t.Fatalf("readBinaryWatch(1) = %v, want %v", got, want)
	}
	for i := range want {
		if got[i] != want[i] {
			t.Fatalf("readBinaryWatch(1) = %v, want %v", got, want)
		}
	}
	if got := readBinaryWatch(9); len(got) != 0 {
		t.Fatalf("readBinaryWatch(9) = %v, want none", got)
	}
}
