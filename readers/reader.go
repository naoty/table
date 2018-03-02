package readers

import "github.com/naoty/table/table"

// Reader is an interface read data from string
type Reader interface {
	ReadHeader(str string) table.Row
	ReadRow(str string) table.Row
}
