package main

import (
	"os"
)

// Name is the name of this application.
const Name = "table"

// Version is the version of this application.
var Version = "0.1.2"

func main() {
	cli := &CLI{inStream: os.Stdin, outStream: os.Stdout, errStream: os.Stdout}
	code := cli.Run(os.Args)
	os.Exit(code)
}
