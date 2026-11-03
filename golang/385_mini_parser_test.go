package leetcode

import "testing"

func TestDeserializeNestedExamples(t *testing.T) {
	got := deserializeNested("324")
	if got.value == nil || *got.value != 324 {
		t.Fatalf("deserializeNested(324) = %v, want 324", got)
	}
	got = deserializeNested("[123,[456,[789]]]")
	if got.value != nil || len(got.list) != 2 || got.list[0].value == nil || *got.list[0].value != 123 {
		t.Fatalf("deserializeNested() top level = %+v", got)
	}
	inner := got.list[1]
	if len(inner.list) != 2 || inner.list[0].value == nil || *inner.list[0].value != 456 {
		t.Fatalf("deserializeNested() inner = %+v", inner)
	}
	deepest := inner.list[1]
	if len(deepest.list) != 1 || deepest.list[0].value == nil || *deepest.list[0].value != 789 {
		t.Fatalf("deserializeNested() deepest = %+v", deepest)
	}
}
