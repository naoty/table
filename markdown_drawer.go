package main

import (
	"fmt"
	"strings"
)

// MarkdownDrawer is a Drawer to draw tables in markdown format.
type MarkdownDrawer struct {
}

// Draw makes a string for a given table in markdown format.
func (drawer MarkdownDrawer) Draw(table *Table) string {
	columnWidths := []int{}
	for _, column := range table.Columns {
		columnWidths = append(columnWidths, column.Width())
	}

	buf := []string{}

	// Header
	if table.Header != nil {
		buf = append(buf, table.Header.Draw(columnWidths))
		buf = append(buf, drawer.drawBorder(table, columnWidths))
	}

	// Each rows
	for _, row := range table.Rows {
		buf = append(buf, row.Draw(columnWidths))
	}

	return fmt.Sprintf("%v\n", strings.Join(buf, "\n"))
}

// DrawBorder makes string represents a border with given column widths.
func (drawer MarkdownDrawer) drawBorder(table *Table, widths []int) string {
	buf := []string{}

	for _, width := range widths {
		buf = append(buf, strings.Repeat("-", width))
	}

	return "| " + strings.Join(buf, " | ") + " |"
}
