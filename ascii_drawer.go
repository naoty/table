package main

import (
	"fmt"
	"strings"
)

// ASCIIDrawer is a Drawer to draw Tables in ASCII format.
type ASCIIDrawer struct {
}

// Draw makes a string for a given Table in ASCII format.
func (drawer ASCIIDrawer) Draw(table *Table) string {
	columnWidths := []int{}
	for _, column := range table.Columns {
		columnWidths = append(columnWidths, column.Width())
	}

	buf := []string{}

	buf = append(buf, drawer.drawBorder(table, columnWidths))

	// Header
	if table.Header != nil {
		buf = append(buf, table.Header.Draw(columnWidths))
		buf = append(buf, drawer.drawBorder(table, columnWidths))
	}

	// Each rows
	for _, row := range table.Rows {
		buf = append(buf, row.Draw(columnWidths))
	}

	buf = append(buf, drawer.drawBorder(table, columnWidths))

	return fmt.Sprintf("%v\n", strings.Join(buf, "\n"))
}

// DrawBorder makes string represents a border with given column widths.
func (drawer ASCIIDrawer) drawBorder(table *Table, widths []int) string {
	buf := []string{}

	for _, width := range widths {
		buf = append(buf, strings.Repeat("-", width+2))
	}

	return "+" + strings.Join(buf, "+") + "+"
}
