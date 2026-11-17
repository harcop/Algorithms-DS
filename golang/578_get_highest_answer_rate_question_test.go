package leetcode

import (
	"strings"
	"testing"
)

func TestHighestAnswerRateSQL(t *testing.T) {
	query := strings.ToLower(highestAnswerRateSQL)
	if !strings.Contains(query, "survey_log") || !strings.Contains(query, "action = 'answer'") {
		t.Fatalf("highestAnswerRateSQL = %q, want the question with the highest answer rate", highestAnswerRateSQL)
	}
}
