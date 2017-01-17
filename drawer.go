package main

// Drawer is the interface to draw a Table.
type Drawer interface {
	Draw(table *Table) string
}
