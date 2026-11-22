package leetcode

// LeetCode #635 - Design Log Storage System
//
// Timestamps are zero-padded, so cutting them to the granularity keeps string order.
type logEntry struct {
	id int
	ts string
}

type LogSystem struct {
	logs []logEntry
}

func NewLogSystem() *LogSystem {
	return &LogSystem{}
}

func (ls *LogSystem) Put(id int, timestamp string) {
	ls.logs = append(ls.logs, logEntry{id: id, ts: timestamp})
}

func (ls *LogSystem) Retrieve(start string, end string, granularity string) []int {
	size := logPrecision(granularity)
	start, end = start[:size], end[:size]
	var ids []int
	for _, entry := range ls.logs {
		stamp := entry.ts[:size]
		if stamp >= start && stamp <= end {
			ids = append(ids, entry.id)
		}
	}
	return ids
}

func logPrecision(granularity string) int {
	switch granularity {
	case "Year":
		return 4
	case "Month":
		return 7
	case "Day":
		return 10
	case "Hour":
		return 13
	case "Minute":
		return 16
	default:
		return 19
	}
}
