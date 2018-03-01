package drawer

import (
	"fmt"
	"strings"

	"github.com/naoty/table/table"
	"github.com/naoty/table/utils"
)

// ConfluenceDrawer is a Drawer to draw tables in Confluence Wiki Markup.
type ConfluenceDrawer struct {
}

// Draw makes a string for a given table in Confluence Wiki Markup.
func (drawer ConfluenceDrawer) Draw(table *table.Table) string {
	columnWidths := []int{}
	for _, column := range table.Columns {
		columnWidths = append(columnWidths, column.Width())
	}

	buf := []string{}

	// Header
	if table.Header != nil {
		buf = append(buf, drawer.DrawHeader(table.Header, columnWidths))
	}

	// Each rows
	for _, row := range table.Rows {
		buf = append(buf, drawer.DrawRow(row, columnWidths))
	}

	return fmt.Sprintf("%v\n", strings.Join(buf, "\n"))
}

// DrawHeader makes a string representing a header with given column widths.
func (drawer ConfluenceDrawer) DrawHeader(row table.Row, widths []int) string {
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

// DrawRow makes a string representing a row with given column widths.
func (drawer ConfluenceDrawer) DrawRow(row table.Row, widths []int) string {
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
