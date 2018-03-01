package table

import "github.com/naoty/table/utils"

// Column represents a column in Table.
type Column []string

// Width caluculates the width of a Column.
func (column Column) Width() int {
	max := 0
	for _, item := range column {
		len := utils.LenInHalfSize(item)
		if len > max {
			max = len
		}
	}
	return max
}
