package main

import (
	"os"

	"golang.org/x/text/width"
)

// Name is the name of this application.
const Name = "table"

// Version is the version of this application.
var Version = "0.1.0"

func main() {
	cli := &CLI{inStream: os.Stdin, outStream: os.Stdout, errStream: os.Stdout}
	code := cli.Run(os.Args)
	os.Exit(code)
}

func lenInHalfSize(str string) int {
	len := 0

	for _, r := range str {
		p := width.LookupRune(r)
		switch p.Kind() {
		case width.EastAsianFullwidth, width.EastAsianWide:
			len += 2
		default:
			len++
		}
	}

	return len
}
