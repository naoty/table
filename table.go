package main

import (
	"fmt"
	"strings"
)

// Table represents a dataset in a table.
type Table struct {
	Header  Row
	Rows    []Row
	Columns []Column
}

// NewTable makes a new Table.
func NewTable() *Table {
	return &Table{
		Rows:    []Row{},
		Columns: []Column{},
	}
}

// AppendHeader parses given string and set it to the header.
func (table *Table) AppendHeader(str string) {
	items := strings.Split(str, "\t")

	for i, item := range items {
		if len(table.Columns) > i {
			table.Columns[i] = append(table.Columns[i], item)
		} else {
			table.Columns = append(table.Columns, Column{item})
		}
	}

	table.Header = items
}

// AppendRow parses given string and appends row into the table.
func (table *Table) AppendRow(str string) {
	items := strings.Split(str, "\t")

	for i, item := range items {
		if len(table.Columns) > i {
			table.Columns[i] = append(table.Columns[i], item)
		} else {
			table.Columns = append(table.Columns, Column{item})
		}
	}

	table.Rows = append(table.Rows, items)
}

func (table *Table) String() string {
	columnWidths := []int{}
	for _, column := range table.Columns {
		columnWidths = append(columnWidths, column.Width())
	}

	buf := []string{}

	buf = append(buf, table.DrawBorder(columnWidths))

	// Header
	if table.Header != nil {
		buf = append(buf, table.Header.Draw(columnWidths))
		buf = append(buf, table.DrawBorder(columnWidths))
	}

	// Each rows
	for _, row := range table.Rows {
		buf = append(buf, row.Draw(columnWidths))
	}

	buf = append(buf, table.DrawBorder(columnWidths))

	return fmt.Sprintf("%v\n", strings.Join(buf, "\n"))
}

// DrawBorder makes string represents a border with given column widths.
func (table *Table) DrawBorder(widths []int) string {
	buf := []string{}

	for _, width := range widths {
		buf = append(buf, strings.Repeat("-", width+2))
	}

	return "+" + strings.Join(buf, "+") + "+"
}
