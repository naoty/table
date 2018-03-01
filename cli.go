package main

import (
	"bufio"
	"flag"
	"fmt"
	"io"

	"github.com/naoty/table/drawers"
	"github.com/naoty/table/table"
)

// Exit codes represent exit codes for particular situations.
const (
	ExitCodeOK = 0

	ExitCodeError = 10 + iota
)

// Format option values
const (
	FormatOptionASCII      = "ascii"
	FormatOptionMarkdown   = "markdown"
	FormatOptionConfluence = "confluence"
)

// CLI represents the CLI for this application.
type CLI struct {
	inStream             io.Reader
	outStream, errStream io.Writer
}

// Run runs commands with given args.
func (cli *CLI) Run(args []string) int {
	flags := flag.NewFlagSet(Name, flag.ContinueOnError)

	var version bool
	flags.BoolVar(&version, "version", false, "Print the version of this application")
	flags.BoolVar(&version, "v", false, "Print the version of this application")

	var header bool
	flags.BoolVar(&header, "header", false, "Parse the first row as a header")
	flags.BoolVar(&header, "H", false, "Parse the first row as a header")

	var format string
	flags.StringVar(&format, "format", "ascii", "Specify how to format a table (ascii|markdown|confluence)")
	flags.StringVar(&format, "f", "ascii", "Specify how to format a table (ascii|markdown|confluence)")

	flags.Parse(args[1:])

	if version {
		fmt.Fprintln(cli.outStream, Version)
		return ExitCodeOK
	}

	table := table.NewTable()
	scanner := bufio.NewScanner(cli.inStream)
	for i := 0; scanner.Scan(); i++ {
		if header && i == 0 {
			table.AppendHeader(scanner.Text())
		} else {
			table.AppendRow(scanner.Text())
		}
	}
	if err := scanner.Err(); err != nil {
		fmt.Fprintf(cli.errStream, "Failed to scan stdin\n")
		return ExitCodeError
	}

	var drawer drawers.Drawer
	switch format {
	case FormatOptionASCII:
		drawer = drawers.ASCIIDrawer{}
	case FormatOptionMarkdown:
		drawer = drawers.MarkdownDrawer{}
	case FormatOptionConfluence:
		drawer = drawers.ConfluenceDrawer{}
	default:
		drawer = drawers.ASCIIDrawer{}
	}
	fmt.Fprintf(cli.outStream, "%v", drawer.Draw(table))

	return ExitCodeOK
}
