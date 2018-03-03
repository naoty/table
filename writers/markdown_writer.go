package writers

import (
	"fmt"
	"strings"

	"github.com/naoty/table/table"
	"github.com/naoty/table/utils"
)

// MarkdownWriter is a Writer to draw tables in markdown format.
type MarkdownWriter struct {
}

// Write makes a string for a given table in markdown format.
func (writer MarkdownWriter) Write(table *table.Table) string {
	columnWidths := []int{}
	for _, column := range table.Columns {
		columnWidths = append(columnWidths, column.Width())
	}

	buf := []string{}

	// Header
	if table.Header != nil {
		buf = append(buf, writer.WriteRow(table.Header, columnWidths))
		buf = append(buf, writer.WriteBorder(table, columnWidths))
	}

	// Each rows
	for _, row := range table.Rows {
		buf = append(buf, writer.WriteRow(row, columnWidths))
	}

	return fmt.Sprintf("%v\n", strings.Join(buf, "\n"))
}

// WriteRow makes string represents a row with given column widths.
func (writer MarkdownWriter) WriteRow(row table.Row, widths []int) string {
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

// WriteBorder makes string represents a border with given column widths.
func (writer MarkdownWriter) WriteBorder(table *table.Table, widths []int) string {
	buf := []string{}

	for _, width := range widths {
		buf = append(buf, strings.Repeat("-", width))
	}

	return "| " + strings.Join(buf, " | ") + " |"
}
