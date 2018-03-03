package readers

import "github.com/naoty/table/table"

// Reader is an interface read data from string
type Reader interface {
	ReadTable(header bool) (*table.Table, error)
}
