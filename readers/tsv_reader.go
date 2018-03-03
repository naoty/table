package readers

import (
	"encoding/csv"
	"io"

	"github.com/naoty/table/table"
)

// TSVReader is a Reader to read data in TSV format.
type TSVReader struct {
	Reader io.Reader
}

// ReadTable builds and returns a Table.
func (reader TSVReader) ReadTable(header bool) (*table.Table, error) {
	tsvReader := csv.NewReader(reader.Reader)
	tsvReader.Comma = '\t'

	records, err := tsvReader.ReadAll()
	if err != nil {
		return nil, err
	}

	table := table.NewTable()

	if header {
		table.AppendHeader(records[0])
	} else {
		table.AppendRow(records[0])
	}

	for _, record := range records[1:] {
		table.AppendRow(record)
	}

	return table, nil
}
