package leetcode

// LeetCode #158 - Read N Characters Given Read4 II
//
// Keep characters that the last read4 call did not use, so later reads continue in order.
type Read4II struct {
	data  []byte
	pos   int
	stash []byte
}

func NewRead4II(data string) *Read4II {
	return &Read4II{data: []byte(data)}
}

func (reader *Read4II) read4(buf []byte) int {
	n := 0
	for n < 4 && reader.pos < len(reader.data) {
		buf[n] = reader.data[reader.pos]
		reader.pos++
		n++
	}
	return n
}

// Read copies up to n characters into buf and returns how many were copied.
func (reader *Read4II) Read(buf []byte, n int) int {
	written := 0
	tmp := make([]byte, 4)
	for written < n {
		for written < n && len(reader.stash) > 0 {
			buf[written] = reader.stash[0]
			reader.stash = reader.stash[1:]
			written++
		}
		if written >= n {
			break
		}
		got := reader.read4(tmp)
		if got == 0 {
			break
		}
		take := got
		if need := n - written; take > need {
			take = need
		}
		copy(buf[written:written+take], tmp[:take])
		written += take
		if take < got {
			reader.stash = append(reader.stash, tmp[take:got]...)
		}
	}
	return written
}
