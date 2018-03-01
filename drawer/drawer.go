package drawer

import "github.com/naoty/table/table"

// Drawer is the interface to draw a Table.
type Drawer interface {
	Draw(table *table.Table) string
}
