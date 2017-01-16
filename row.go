package main

import "strings"

// Row represents a row in Table.
type Row []string

// Draw makes string representing a row in a table with given column widths.
func (row Row) Draw(widths []int) string {
	buf := []string{"|"}

	for i, item := range row {
		buf = append(buf, " ")
		buf = append(buf, item)

		paddingWidth := widths[i] - len(item)
		buf = append(buf, strings.Repeat(" ", paddingWidth))
		buf = append(buf, " |")
	}

	return strings.Join(buf, "")
}
