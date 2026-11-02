package leetcode

// LeetCode #359 - Logger Rate Limiter
//
// A message prints when it is new or its last print was at least 10 seconds ago.
type Logger struct {
	last map[string]int
}

func NewLogger() *Logger {
	return &Logger{last: map[string]int{}}
}

func (l *Logger) ShouldPrintMessage(timestamp int, message string) bool {
	if previous, ok := l.last[message]; ok && timestamp-previous < 10 {
		return false
	}
	l.last[message] = timestamp
	return true
}
