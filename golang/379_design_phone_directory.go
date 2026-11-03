package leetcode

// LeetCode #379 - Design Phone Directory
//
// A flag records which numbers are already handed out.
type PhoneDirectory struct {
	used []bool
}

func NewPhoneDirectory(maxNumbers int) *PhoneDirectory {
	return &PhoneDirectory{used: make([]bool, maxNumbers)}
}

func (d *PhoneDirectory) Get() int {
	for i, taken := range d.used {
		if !taken {
			d.used[i] = true
			return i
		}
	}
	return -1
}

func (d *PhoneDirectory) Check(number int) bool {
	return number >= 0 && number < len(d.used) && !d.used[number]
}

func (d *PhoneDirectory) Release(number int) {
	if number >= 0 && number < len(d.used) {
		d.used[number] = false
	}
}
