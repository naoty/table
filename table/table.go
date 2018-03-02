package table

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
func (table *Table) AppendHeader(header Row) {
	for i, item := range header {
		if len(table.Columns) > i {
			table.Columns[i] = append(table.Columns[i], item)
		} else {
			table.Columns = append(table.Columns, Column{item})
		}
	}

	table.Header = header
}

// AppendRow parses given string and appends row into the table.
func (table *Table) AppendRow(row Row) {
	for i, item := range row {
		if len(table.Columns) > i {
			table.Columns[i] = append(table.Columns[i], item)
		} else {
			table.Columns = append(table.Columns, Column{item})
		}
	}

	table.Rows = append(table.Rows, row)
}
