package leetcode

import "testing"

func TestLogSystemExample(t *testing.T) {
	logs := NewLogSystem()
	logs.Put(1, "2017:01:01:23:59:59")
	logs.Put(2, "2017:01:01:22:59:59")
	logs.Put(3, "2016:01:01:00:00:00")
	got := logs.Retrieve("2016:01:01:01:01:01", "2017:01:01:23:00:00", "Year")
	if !sameInts(got, []int{1, 2, 3}) {
		t.Fatalf("Retrieve(Year) = %v, want [1 2 3]", got)
	}
	got = logs.Retrieve("2016:01:01:01:01:01", "2017:01:01:23:00:00", "Hour")
	if !sameInts(got, []int{1, 2}) {
		t.Fatalf("Retrieve(Hour) = %v, want [1 2]", got)
	}
}
