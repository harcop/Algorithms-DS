package leetcode

import "testing"

func TestCanAttendMeetingsExampleOne(t *testing.T) {
	intervals := [][]int{{0, 30}, {5, 10}, {15, 20}}
	if canAttendMeetings(intervals) {
		t.Fatal("canAttendMeetings() = true, want false")
	}
}

func TestCanAttendMeetingsExampleTwo(t *testing.T) {
	intervals := [][]int{{7, 10}, {2, 4}}
	if !canAttendMeetings(intervals) {
		t.Fatal("canAttendMeetings() = false, want true")
	}
}
