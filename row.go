package main

import (
	"strings"

	"golang.org/x/text/width"
)

// Row represents a row in Table.
type Row []string

// Draw makes string representing a row in a table with given column widths.
func (row Row) Draw(widths []int) string {
	buf := []string{"|"}

	for i, item := range row {
		buf = append(buf, " ")
		buf = append(buf, item)

		paddingWidth := widths[i] - lenInHalfSize(item)
		buf = append(buf, strings.Repeat(" ", paddingWidth))
		buf = append(buf, " |")
	}

	return strings.Join(buf, "")
}

func lenInHalfSize(str string) int {
	len := 0

	for _, r := range str {
		p := width.LookupRune(r)
		switch p.Kind() {
		case width.EastAsianFullwidth, width.EastAsianWide:
			len += 2
		default:
			len++
		}
	}

	return len
}
