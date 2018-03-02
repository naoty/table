package main

import (
	"bufio"
	"fmt"
	"io"
	"strings"

	"github.com/naoty/table/table"
	"github.com/naoty/table/writers"
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
	var writer writers.Writer = writers.ASCIIWriter{}
	shouldShowHeader := false

	for i, arg := range args {
		switch arg {
		case "--format", "-f":
			if i == len(args)-1 {
				continue
			}

			_, writerName := parseFormat(arg)

			switch writerName {
			case FormatOptionASCII:
				writer = writers.ASCIIWriter{}
			case FormatOptionMarkdown:
				writer = writers.MarkdownWriter{}
			case FormatOptionConfluence:
				writer = writers.ConfluenceWriter{}
			default:
				writer = writers.ASCIIWriter{}
			}
		case "--header", "-H":
			shouldShowHeader = true
		case "--help", "-h":
			fmt.Fprintln(cli.outStream, cli.Help())
			return ExitCodeOK
		case "--version", "-v":
			fmt.Fprintln(cli.outStream, Version)
			return ExitCodeOK
		}
	}

	table := table.NewTable()
	scanner := bufio.NewScanner(cli.inStream)
	for i := 0; scanner.Scan(); i++ {
		if shouldShowHeader && i == 0 {
			table.AppendHeader(scanner.Text())
		} else {
			table.AppendRow(scanner.Text())
		}
	}

	if err := scanner.Err(); err != nil {
		fmt.Fprintln(cli.errStream, err)
		return ExitCodeError
	}

	fmt.Fprintf(cli.outStream, "%v", writer.Write(table))
	return ExitCodeOK
}

// Help returns help messages.
func (cli *CLI) Help() string {
	indent := strings.Repeat(" ", 2)

	lines := []string{}
	lines = append(lines, "Usage:")
	lines = append(lines, fmt.Sprintf("%stable [--header | -H] [--format | -f (ascii|markdown|confluence)]", indent))
	lines = append(lines, fmt.Sprintf("%stable --help | -h", indent))
	lines = append(lines, fmt.Sprintf("%stable --version | -v", indent))
	lines = append(lines, "")
	lines = append(lines, "Options:")
	lines = append(lines, fmt.Sprintf("%s--header, -H: Show header", indent))
	lines = append(lines, fmt.Sprintf("%s--format, -f [ascii|markdown|confluence]: Set format (default: ascii)", indent))
	lines = append(lines, fmt.Sprintf("%s--help, -h: Show version number", indent))
	lines = append(lines, fmt.Sprintf("%s--version, -v: Show version number", indent))

	return strings.Join(lines, "\n")
}

func parseFormat(format string) (string, string) {
	items := strings.Split(format, ":")
	if len(items) == 0 {
		return "", ""
	} else if len(items) == 1 {
		return "", items[0]
	} else {
		return items[0], items[1]
	}
}
