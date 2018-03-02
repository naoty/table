package readers

import (
	"strings"

	"github.com/naoty/table/table"
)

// TSVReader is a Reader to read data in TSV format.
type TSVReader struct {
}

// ReadHeader returns a Row separated by tabs.
func (reader TSVReader) ReadHeader(str string) table.Row {
	return strings.Split(str, "\t")
}

// ReadRow returns a Row separated by tabs.
func (reader TSVReader) ReadRow(str string) table.Row {
	return strings.Split(str, "\t")
}
