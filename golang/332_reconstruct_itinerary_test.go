package leetcode

import (
	"strings"
	"testing"
)

func TestFindItineraryExamples(t *testing.T) {
	got := findItinerary([][]string{
		{"MUC", "LHR"},
		{"JFK", "MUC"},
		{"SFO", "SJC"},
		{"LHR", "SFO"},
	})
	if strings.Join(got, ",") != "JFK,MUC,LHR,SFO,SJC" {
		t.Fatalf("findItinerary() = %v, want JFK MUC LHR SFO SJC", got)
	}

	got = findItinerary([][]string{
		{"JFK", "SFO"},
		{"JFK", "ATL"},
		{"SFO", "ATL"},
		{"ATL", "JFK"},
		{"ATL", "SFO"},
	})
	if strings.Join(got, ",") != "JFK,ATL,JFK,SFO,ATL,SFO" {
		t.Fatalf("findItinerary() = %v, want JFK ATL JFK SFO ATL SFO", got)
	}
}
