package leetcode

// LeetCode #346 - Moving Average from Data Stream
//
// A fixed window keeps the running sum and drops the oldest value once it is full.
type MovingAverage struct {
	window []int
	size   int
	sum    int64
}

func NewMovingAverage(size int) *MovingAverage {
	return &MovingAverage{size: size}
}

func (m *MovingAverage) Next(val int) float64 {
	m.window = append(m.window, val)
	m.sum += int64(val)
	if len(m.window) > m.size {
		m.sum -= int64(m.window[0])
		m.window = m.window[1:]
	}
	return float64(m.sum) / float64(len(m.window))
}
