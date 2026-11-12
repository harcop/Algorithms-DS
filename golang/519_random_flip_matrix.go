package leetcode

// LeetCode #519 - Random Flip Matrix
//
// A shrinking range of unused indexes is swapped with a map so each remaining cell is equally likely.
type RandomFlipMatrix struct {
	rows, cols int
	remaining  int
	mapped     map[int]int
	rng        uint64
}

func NewRandomFlipMatrix(m, n int) *RandomFlipMatrix {
	return &RandomFlipMatrix{
		rows:      m,
		cols:      n,
		remaining: m * n,
		mapped:    map[int]int{},
		rng:       0x9e3779b97f4a7c15,
	}
}

func (m *RandomFlipMatrix) Flip() []int {
	index := int(m.nextUint() % uint64(m.remaining))
	m.remaining--
	chosen := index
	if value, ok := m.mapped[index]; ok {
		chosen = value
	}
	last := m.remaining
	if value, ok := m.mapped[m.remaining]; ok {
		last = value
	}
	m.mapped[index] = last
	return []int{chosen / m.cols, chosen % m.cols}
}

func (m *RandomFlipMatrix) Reset() {
	m.remaining = m.rows * m.cols
	m.mapped = map[int]int{}
}

func (m *RandomFlipMatrix) nextUint() uint64 {
	m.rng ^= m.rng << 7
	m.rng ^= m.rng >> 9
	m.rng ^= m.rng << 8
	return m.rng
}
