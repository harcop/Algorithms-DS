package leetcode

// LeetCode #157 - Read N Characters Given Read4
//
// Read the file four characters at a time and copy at most n of them.
type Read4 struct {
	data  []byte
	pos   int
	stash []byte
}

func NewRead4(data string) *Read4 {
	return &Read4{data: []byte(data)}
}

func (reader *Read4) read4(buf []byte) int {
	n := 0
	for n < 4 && reader.pos < len(reader.data) {
		buf[n] = reader.data[reader.pos]
		reader.pos++
		n++
	}
	return n
}

// Read copies up to n characters into buf and returns how many were copied.
func (reader *Read4) Read(buf []byte, n int) int {
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
