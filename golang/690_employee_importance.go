package leetcode

// Employee is a worker and the ids of the people who report to them.
type Employee struct {
	Id           int
	Importance   int
	Subordinates []int
}

// LeetCode #690 - Employee Importance
//
// The total is this employee's importance plus the same total for every subordinate.
func getImportance(employees []*Employee, id int) int {
	byID := make(map[int]*Employee, len(employees))
	for _, employee := range employees {
		byID[employee.Id] = employee
	}
	var total func(int) int
	total = func(id int) int {
		employee := byID[id]
		sum := employee.Importance
		for _, subordinate := range employee.Subordinates {
			sum += total(subordinate)
		}
		return sum
	}
	return total(id)
}
