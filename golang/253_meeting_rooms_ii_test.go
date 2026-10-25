package leetcode

import "testing"

func TestMinMeetingRoomsExampleOne(t *testing.T) {
	intervals := [][]int{{0, 30}, {5, 10}, {15, 20}}
	if got := minMeetingRooms(intervals); got != 2 {
		t.Fatalf("minMeetingRooms() = %d, want 2", got)
	}
}

func TestMinMeetingRoomsExampleTwo(t *testing.T) {
	intervals := [][]int{{7, 10}, {2, 4}}
	if got := minMeetingRooms(intervals); got != 1 {
		t.Fatalf("minMeetingRooms() = %d, want 1", got)
	}
}
