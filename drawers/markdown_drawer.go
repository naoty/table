package drawers

import (
	"fmt"
	"strings"

	"github.com/naoty/table/table"
	"github.com/naoty/table/utils"
)

// MarkdownDrawer is a Drawer to draw tables in markdown format.
type MarkdownDrawer struct {
}

// Draw makes a string for a given table in markdown format.
func (drawer MarkdownDrawer) Draw(table *table.Table) string {
	columnWidths := []int{}
	for _, column := range table.Columns {
		columnWidths = append(columnWidths, column.Width())
	}

	buf := []string{}

	// Header
	if table.Header != nil {
		buf = append(buf, drawer.DrawRow(table.Header, columnWidths))
		buf = append(buf, drawer.DrawBorder(table, columnWidths))
	}

	// Each rows
	for _, row := range table.Rows {
		buf = append(buf, drawer.DrawRow(row, columnWidths))
	}

	return fmt.Sprintf("%v\n", strings.Join(buf, "\n"))
}

// DrawRow makes string represents a row with given column widths.
func (drawer MarkdownDrawer) DrawRow(row table.Row, widths []int) string {
	buf := []string{"|"}

	for i, item := range row {
		buf = append(buf, " ")
		buf = append(buf, item)

		paddingWidth := widths[i] - utils.LenInHalfSize(item)
		buf = append(buf, strings.Repeat(" ", paddingWidth))
		buf = append(buf, " |")
	}

	return strings.Join(buf, "")
}

// DrawBorder makes string represents a border with given column widths.
func (drawer MarkdownDrawer) DrawBorder(table *table.Table, widths []int) string {
	buf := []string{}

	for _, width := range widths {
		buf = append(buf, strings.Repeat("-", width))
	}

	return "| " + strings.Join(buf, " | ") + " |"
}
