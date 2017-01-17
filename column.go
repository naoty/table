package main

// Column represents a column in Table.
type Column []string

// Width caluculates the width of a Column.
func (column Column) Width() int {
	max := 0
	for _, item := range column {
		len := lenInHalfSize(item)
		if len > max {
			max = len
		}
	}
	return max
}
