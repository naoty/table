package writers

import "github.com/naoty/table/table"

// Writer is the interface to draw a Table.
type Writer interface {
	Write(table *table.Table) string
}
