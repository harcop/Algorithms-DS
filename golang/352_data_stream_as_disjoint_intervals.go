package leetcode

import "sort"

// LeetCode #352 - Data Stream as Disjoint Intervals
//
// Each new value extends, joins, or opens an interval kept in start order.
type SummaryRanges struct {
	starts []int
	ends   []int
}

func NewSummaryRanges() *SummaryRanges {
	return &SummaryRanges{}
}

func (s *SummaryRanges) AddNum(value int) {
	i := sort.SearchInts(s.starts, value)
	if i < len(s.starts) && s.starts[i] == value {
		return
	}
	if i > 0 && value <= s.ends[i-1] {
		return
	}
	joinLeft := i > 0 && s.ends[i-1]+1 == value
	joinRight := i < len(s.starts) && s.starts[i] == value+1
	switch {
	case joinLeft && joinRight:
		s.ends[i-1] = s.ends[i]
		s.starts = append(s.starts[:i], s.starts[i+1:]...)
		s.ends = append(s.ends[:i], s.ends[i+1:]...)
	case joinLeft:
		s.ends[i-1] = value
	case joinRight:
		s.starts[i] = value
	default:
		s.starts = insertInt(s.starts, i, value)
		s.ends = insertInt(s.ends, i, value)
	}
}

func (s *SummaryRanges) GetIntervals() [][]int {
	out := make([][]int, len(s.starts))
	for i := range s.starts {
		out[i] = []int{s.starts[i], s.ends[i]}
	}
	return out
}

func insertInt(values []int, at, value int) []int {
	values = append(values, 0)
	copy(values[at+1:], values[at:])
	values[at] = value
	return values
}
