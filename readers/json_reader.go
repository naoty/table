package readers

import (
	"encoding/json"
	"fmt"
	"io"

	"github.com/naoty/table/table"
)

// JSONReader is a Reader to read data in JSON format.
type JSONReader struct {
	Reader io.Reader
}

// ReadTable builds and returns a Table.
func (reader JSONReader) ReadTable(header bool) (*table.Table, error) {
	objects := []map[string]interface{}{}
	decoder := json.NewDecoder(reader.Reader)
	if err := decoder.Decode(&objects); err != nil {
		return nil, err
	}

	t := table.NewTable()

	headerFields := []string{}
	for _, object := range objects {
		for key := range object {
			if !contains(headerFields, key) {
				headerFields = append(headerFields, key)
			}
		}
	}
	t.AppendHeader(headerFields)

	for _, object := range objects {
		rowFields := make([]string, len(headerFields))
		for key, value := range object {
			index := position(headerFields, key)
			rowFields[index] = fmt.Sprintf("%v", value)
		}
		t.AppendRow(rowFields)
	}

	return t, nil
}

func contains(strs []string, str string) bool {
	for _, s := range strs {
		if s == str {
			return true
		}
	}
	return false
}

func position(strs []string, str string) int {
	for i, s := range strs {
		if s == str {
			return i
		}
	}
	return -1
}
