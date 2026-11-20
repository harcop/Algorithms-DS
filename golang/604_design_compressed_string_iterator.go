package leetcode

// LeetCode #604 - Design Compressed String Iterator
//
// A letter and its count stay current until that run is spent, then the next run is read.
type StringIterator struct {
	compressed string
	index      int
	char       byte
	remain     int
}

func NewStringIterator(compressed string) *StringIterator {
	return &StringIterator{compressed: compressed}
}

func (it *StringIterator) load() {
	if it.remain > 0 || it.index >= len(it.compressed) {
		return
	}
	it.char = it.compressed[it.index]
	it.index++
	count := 0
	for it.index < len(it.compressed) && it.compressed[it.index] >= '0' && it.compressed[it.index] <= '9' {
		count = count*10 + int(it.compressed[it.index]-'0')
		it.index++
	}
	it.remain = count
}

func (it *StringIterator) Next() byte {
	it.load()
	if it.remain == 0 {
		return ' '
	}
	it.remain--
	return it.char
}

func (it *StringIterator) HasNext() bool {
	it.load()
	return it.remain > 0
}
