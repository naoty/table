package readers

import (
	"io"

	"github.com/naoty/table/table"
)

// JSONReader is a Reader to read data in JSON format.
type JSONReader struct {
	Reader io.Reader
}

// ReadTable builds and returns a Table.
func (reader JSONReader) ReadTable(header bool) (*table.Table, error) {
	return table.NewTable(), nil
}
