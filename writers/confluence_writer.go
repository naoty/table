package writers

import (
	"fmt"
	"strings"

	"github.com/naoty/table/table"
	"github.com/naoty/table/utils"
)

// ConfluenceWriter is a Writer to draw tables in Confluence Wiki Markup.
type ConfluenceWriter struct {
}

// Write makes a string for a given table in Confluence Wiki Markup.
func (writer ConfluenceWriter) Write(table *table.Table) string {
	columnWidths := []int{}
	for _, column := range table.Columns {
		columnWidths = append(columnWidths, column.Width())
	}

	buf := []string{}

	// Header
	if table.Header != nil {
		buf = append(buf, writer.WriteHeader(table.Header, columnWidths))
	}

	// Each rows
	for _, row := range table.Rows {
		buf = append(buf, writer.WriteRow(row, columnWidths))
	}

	return fmt.Sprintf("%v\n", strings.Join(buf, "\n"))
}

// WriteHeader makes a string representing a header with given column widths.
func (writer ConfluenceWriter) WriteHeader(row table.Row, widths []int) string {
	buf := []string{"|"}

	for i, item := range row {
		buf = append(buf, "|")
		buf = append(buf, item)

		paddingWidth := widths[i] - utils.LenInHalfSize(item)
		buf = append(buf, strings.Repeat(" ", paddingWidth))
		buf = append(buf, " |")
	}

	return strings.Join(buf, "")
}

// WriteRow makes a string representing a row with given column widths.
func (writer ConfluenceWriter) WriteRow(row table.Row, widths []int) string {
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
