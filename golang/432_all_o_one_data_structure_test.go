package leetcode

import "testing"

func TestAllOneExample(t *testing.T) {
	counter := NewAllOne()
	if counter.GetMaxKey() != "" || counter.GetMinKey() != "" {
		t.Fatal("empty counter returned a key")
	}
	counter.Inc("hello")
	counter.Inc("hello")
	if counter.GetMaxKey() != "hello" || counter.GetMinKey() != "hello" {
		t.Fatalf("after hello twice, max=%q min=%q", counter.GetMaxKey(), counter.GetMinKey())
	}
	counter.Inc("leet")
	if counter.GetMaxKey() != "hello" || counter.GetMinKey() != "leet" {
		t.Fatalf("after leet, max=%q min=%q", counter.GetMaxKey(), counter.GetMinKey())
	}
	counter.Dec("hello")
	counter.Dec("hello")
	if counter.GetMaxKey() != "leet" || counter.GetMinKey() != "leet" {
		t.Fatalf("after removing hello, max=%q min=%q", counter.GetMaxKey(), counter.GetMinKey())
	}
	counter.Dec("leet")
	if counter.GetMaxKey() != "" || counter.GetMinKey() != "" {
		t.Fatal("counter should be empty after the last decrement")
	}

	counter.Inc("a")
	counter.Inc("b")
	counter.Inc("a")
	counter.Inc("b")
	counter.Dec("a")
	if counter.GetMinKey() != "a" || counter.GetMaxKey() != "b" {
		t.Fatalf("shared buckets, min=%q max=%q", counter.GetMinKey(), counter.GetMaxKey())
	}
	counter.Inc("a")
	if counter.GetMinKey() != counter.GetMaxKey() || (counter.GetMinKey() != "a" && counter.GetMinKey() != "b") {
		t.Fatalf("tied frequencies returned %q and %q", counter.GetMinKey(), counter.GetMaxKey())
	}
}
