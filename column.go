package main

// Column represents a column in Table.
type Column []string

// Width caluculates the width of a Column.
func (column Column) Width() int {
	max := 0
	for _, item := range column {
		if len(item) > max {
			max = len(item)
		}
	}
	return max
}
