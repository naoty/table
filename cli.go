package main

import (
	"bufio"
	"flag"
	"fmt"
	"io"
)

// Exit codes represent exit codes for particular situations.
const (
	ExitCodeOK = 0

	ExitCodeError = 10 + iota
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

	flags.Parse(args[1:])

	if version {
		fmt.Fprintln(cli.outStream, Version)
		return ExitCodeOK
	}

	table := NewTable()
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

	fmt.Fprintf(cli.outStream, "%v", table)

	return ExitCodeOK
}
